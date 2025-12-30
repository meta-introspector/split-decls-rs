use std::collections::HashMap;
use std::path::Path;
use syn::{visit_mut::VisitMut, *};
use quote::{quote, ToTokens};
use proc_macro2::TokenStream;
use anyhow::Result;
use serde::{Deserialize, Serialize};
//use crate::syscall_prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyscallInterceptor {
    pub syscall_mappings: HashMap<String, SyscallWrapper>,
    pub mock_mode: bool,
    pub dao_governance: bool,
    pub type_safety_level: TypeSafetyLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyscallWrapper {
    pub original_call: String,
    pub wrapper_macro: String,
    pub oracle_type: OracleType,
    pub safety_wrapper: String,
    pub mock_implementation: Option<String>,
    pub dao_policy: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OracleType {
    FileSystem,
    Network,
    Process,
    Memory,
    Time,
    Crypto,
    Custom(String),
}

impl ToTokens for OracleType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = match self {
            OracleType::FileSystem => "FileSystem",
            OracleType::Network => "Network", 
            OracleType::Process => "Process",
            OracleType::Memory => "Memory",
            OracleType::Time => "Time",
            OracleType::Crypto => "Crypto",
            OracleType::Custom(s) => return s.to_tokens(tokens),
        };
        tokens.extend(quote::quote! { #name });
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeSafetyLevel {
    Permissive,  // Log only
    Strict,      // Type check and validate
    Paranoid,    // Full DAO governance required
}

pub struct SyscallAstTransformer {
    interceptor: SyscallInterceptor,
    transformations: HashMap<String, usize>,
}

impl SyscallAstTransformer {
    pub fn new(interceptor: SyscallInterceptor) -> Self {
        Self {
            interceptor,
            transformations: HashMap::new(),
        }
    }
    
    pub fn transform_file(&mut self, file_path: &Path) -> Result<String> {
        let content = 
    std::fs::read_to_string(file_path)?;
        let mut syntax_tree: File = syn::parse_str(&content)?;
        
        self.visit_file_mut(&mut syntax_tree);
        
        Ok(syntax_tree.to_token_stream().to_string())
    }
    
    fn wrap_syscall(&mut self, call_expr: &Expr, wrapper: &SyscallWrapper) -> TokenStream {
        let original = call_expr.to_token_stream();
        let wrapper_macro = &wrapper.wrapper_macro;
        let oracle_type = &wrapper.oracle_type;
        let safety_wrapper = &wrapper.safety_wrapper;
        
        *self.transformations.entry(wrapper.original_call.clone()).or_insert(0) += 1;
        
        match (&wrapper.mock_implementation, self.interceptor.mock_mode) {
            (Some(mock_impl), true) => {
                let mock_tokens: TokenStream = mock_impl.parse().unwrap_or_else(|_| quote! { mock_default() });
                quote! {
                    #wrapper_macro! {
                        oracle_type: #oracle_type,
                        safety_wrapper: #safety_wrapper,
                        mock: #mock_tokens,
                        original: #original
                    }
                }
            }
            _ => {
                match self.interceptor.type_safety_level {
                    TypeSafetyLevel::Permissive => {
                        quote! {
                            #wrapper_macro! {
                                log_syscall(#original)
                            }
                        }
                    }
                    TypeSafetyLevel::Strict => {
                        quote! {
                            #wrapper_macro! {
                                type_safe_syscall::<#oracle_type>(#original)
                            }
                        }
                    }
                    TypeSafetyLevel::Paranoid => {
                        let default_policy = "default_policy".to_string();
                        let dao_policy = wrapper.dao_policy.as_ref().unwrap_or(&default_policy);
                        quote! {
                            #wrapper_macro! {
                                dao_governed_syscall::<#oracle_type>(
                                    #original,
                                    policy: #dao_policy,
                                    oracle: #safety_wrapper
                                )
                            }
                        }
                    }
                }
            }
        }
    }
    
    fn is_syscall_function(&self, path: &syn::Path) -> Option<&SyscallWrapper> {
        let path_str = path.segments.last()?.ident.to_string();
        
        // Check for common syscall patterns
        for (pattern, wrapper) in &self.interceptor.syscall_mappings {
            if path_str.contains(pattern) {
                return Some(wrapper);
            }
        }
        
        None
    }
}

impl VisitMut for SyscallAstTransformer {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        match expr {
            Expr::Call(call) => {
                if let Expr::Path(path_expr) = &*call.func {
                    if let Some(wrapper) = self.is_syscall_function(&path_expr.path).cloned() {
                        let wrapped = self.wrap_syscall(expr, &wrapper);
                        if let Ok(new_expr) = syn::parse2::<Expr>(wrapped) {
                            *expr = new_expr;
                            return;
                        }
                    }
                }
            }
            Expr::MethodCall(method_call) => {
                let method_name = method_call.method.to_string();
                if let Some(wrapper) = self.interceptor.syscall_mappings.get(&method_name).cloned() {
                    let wrapped = self.wrap_syscall(expr, &wrapper);
                    if let Ok(new_expr) = syn::parse2::<Expr>(wrapped) {
                        *expr = new_expr;
                        return;
                    }
                }
            }
            _ => {}
        }
        
        syn::visit_mut::visit_expr_mut(self, expr);
    }
}

pub fn create_default_syscall_interceptor() -> SyscallInterceptor {
    let mut syscall_mappings = HashMap::new();
    
    // File system operations
    syscall_mappings.insert(
        "#[syscall=\"read\"]\nstd::fs::read".to_string(), 
        SyscallWrapper {
            original_call: "#[syscall=\"read\"]\nstd::fs::read".to_string(),
            wrapper_macro: "safe_fs_read".to_string(),
            oracle_type: OracleType::FileSystem,
            safety_wrapper: "FileSystemOracle".to_string(),
            mock_implementation: Some("mock_file_read()".to_string()),
            dao_policy: Some("filesystem_read_policy".to_string()),
        }
    );
    
    syscall_mappings.insert("std::fs::write".to_string(), SyscallWrapper {
        original_call: "std::fs::write".to_string(),
        wrapper_macro: "safe_fs_write".to_string(),
        oracle_type: OracleType::FileSystem,
        safety_wrapper: "FileSystemOracle".to_string(),
        mock_implementation: Some("mock_file_write()".to_string()),
        dao_policy: Some("filesystem_write_policy".to_string()),
    });
    
    // Process operations
    syscall_mappings.insert("std::process::Command".to_string(), SyscallWrapper {
        original_call: "std::process::Command".to_string(),
        wrapper_macro: "safe_process_exec".to_string(),
        oracle_type: OracleType::Process,
        safety_wrapper: "ProcessOracle".to_string(),
        mock_implementation: Some("mock_process_exec()".to_string()),
        dao_policy: Some("process_exec_policy".to_string()),
    });
    
    // Network operations
    syscall_mappings.insert("std::net::TcpStream".to_string(), SyscallWrapper {
        original_call: "std::net::TcpStream".to_string(),
        wrapper_macro: "safe_network_connect".to_string(),
        oracle_type: OracleType::Network,
        safety_wrapper: "NetworkOracle".to_string(),
        mock_implementation: Some("mock_tcp_connect()".to_string()),
        dao_policy: Some("network_connect_policy".to_string()),
    });
    
    // Memory operations
    syscall_mappings.insert("libc::malloc".to_string(), SyscallWrapper {
        original_call: "libc::malloc".to_string(),
        wrapper_macro: "safe_memory_alloc".to_string(),
        oracle_type: OracleType::Memory,
        safety_wrapper: "MemoryOracle".to_string(),
        mock_implementation: Some("mock_malloc()".to_string()),
        dao_policy: Some("memory_alloc_policy".to_string()),
    });
    
    // Time operations
    syscall_mappings.insert("std::time::SystemTime".to_string(), SyscallWrapper {
        original_call: "std::time::SystemTime".to_string(),
        wrapper_macro: "safe_time_access".to_string(),
        oracle_type: OracleType::Time,
        safety_wrapper: "TimeOracle".to_string(),
        mock_implementation: Some("mock_system_time()".to_string()),
        dao_policy: Some("time_access_policy".to_string()),
    });
    
    SyscallInterceptor {
        syscall_mappings,
        mock_mode: false,
        dao_governance: true,
        type_safety_level: TypeSafetyLevel::Strict,
    }
}

pub fn generate_oracle_types() -> TokenStream {
    quote! {
        use std::marker::PhantomData;
        use serde::{Serialize, Deserialize};
        
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct OracleResult<T> {
            pub value: T,
            pub safety_check: bool,
            pub dao_approved: bool,
            pub audit_trail: Vec<String>,
        }
        
        pub trait SystemOracle<T> {
            fn validate(&self, input: &T) -> Result<(), String>;
            fn transform(&self, input: T) -> OracleResult<T>;
            fn audit_log(&self, operation: &str, result: &OracleResult<T>);
        }
        
        #[derive(Debug)]
        pub struct FileSystemOracle;
        
        impl SystemOracle<std::path::PathBuf> for FileSystemOracle {
            fn validate(&self, path: &std::path::PathBuf) -> Result<(), String> {
                if path.to_string_lossy().contains("..") {
                    return Err("Path traversal detected".to_string());
                }
                Ok(())
            }
            
            fn transform(&self, path: std::path::PathBuf) -> OracleResult<std::path::PathBuf> {
                OracleResult {
                    value: path,
                    safety_check: true,
                    dao_approved: true,
                    audit_trail: vec!["filesystem_access".to_string()],
                }
            }
            
            fn audit_log(&self, operation: &str, result: &OracleResult<std::path::PathBuf>) {
                println!("🔍 FileSystem Oracle: {} -> {:?}", operation, result.value);
            }
        }
        
        #[derive(Debug)]
        pub struct ProcessOracle;
        
        impl SystemOracle<String> for ProcessOracle {
            fn validate(&self, command: &String) -> Result<(), String> {
                let dangerous_commands = ["rm", "dd", "format", "del"];
                for dangerous in &dangerous_commands {
                    if command.contains(dangerous) {
                        return Err(format!("Dangerous command detected: {}", dangerous));
                    }
                }
                Ok(())
            }
            
            fn transform(&self, command: String) -> OracleResult<String> {
                OracleResult {
                    value: command,
                    safety_check: true,
                    dao_approved: true,
                    audit_trail: vec!["process_exec".to_string()],
                }
            }
            
            fn audit_log(&self, operation: &str, result: &OracleResult<String>) {
                println!("⚡ Process Oracle: {} -> {}", operation, result.value);
            }
        }
        
        #[derive(Debug)]
        pub struct NetworkOracle;
        
        impl SystemOracle<std::net::SocketAddr> for NetworkOracle {
            fn validate(&self, addr: &std::net::SocketAddr) -> Result<(), String> {
                if addr.ip().is_loopback() {
                    return Ok(());
                }
                // Add more network validation logic
                Ok(())
            }
            
            fn transform(&self, addr: std::net::SocketAddr) -> OracleResult<std::net::SocketAddr> {
                OracleResult {
                    value: addr,
                    safety_check: true,
                    dao_approved: true,
                    audit_trail: vec!["network_connect".to_string()],
                }
            }
            
            fn audit_log(&self, operation: &str, result: &OracleResult<std::net::SocketAddr>) {
                println!("🌐 Network Oracle: {} -> {}", operation, result.value);
            }
        }
        
        // Macro definitions for syscall wrapping
        macro_rules! safe_fs_read {
            ($path:expr) => {{
                let oracle = FileSystemOracle;
                let path_buf = std::path::PathBuf::from($path);
                match oracle.validate(&path_buf) {
                    Ok(_) => {
                        let result = oracle.transform(path_buf);
                        oracle.audit_log("fs_read", &result);
                        
    std::fs::read(result.value)
                    }
                    Err(e) => Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, e))
                }
            }};
        }
        
        macro_rules! safe_fs_write {
            ($path:expr, $contents:expr) => {{
                let oracle = FileSystemOracle;
                let path_buf = std::path::PathBuf::from($path);
                match oracle.validate(&path_buf) {
                    Ok(_) => {
                        let result = oracle.transform(path_buf);
                        oracle.audit_log("fs_write", &result);
                        
    std::fs::write(result.value, $contents)
                    }
                    Err(e) => Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, e))
                }
            }};
        }
        
        macro_rules! safe_process_exec {
            ($command:expr) => {{
                let oracle = ProcessOracle;
                let cmd_string = $command.to_string();
                match oracle.validate(&cmd_string) {
                    Ok(_) => {
                        let result = oracle.transform(cmd_string);
                        oracle.audit_log("process_exec", &result);
                        std::process::
    Command::new(&result.value)
                    }
                    Err(e) => {
                        panic!("Process execution blocked: {}", e);
                    }
                }
            }};
        }
        
        // DAO Governance types
        #[derive(Debug, Serialize, Deserialize)]
        pub struct DaoPolicy {
            pub name: String,
            pub rules: Vec<PolicyRule>,
            pub approval_threshold: f64,
            pub audit_required: bool,
        }
        
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PolicyRule {
            pub condition: String,
            pub action: PolicyAction,
            pub severity: Severity,
        }
        
        #[derive(Debug, Serialize, Deserialize)]
        pub enum PolicyAction {
            Allow,
            Deny,
            RequireApproval,
            Audit,
        }
        
        #[derive(Debug, Serialize, Deserialize)]
        pub enum Severity {
            Low,
            Medium,
            High,
            Critical,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_syscall_interceptor_creation() {
        let interceptor = create_default_syscall_interceptor();
        assert!(!interceptor.syscall_mappings.is_empty());
        assert!(interceptor.dao_governance);
    }
    
    #[test]
    fn test_oracle_types_generation() {
        let oracle_code = generate_oracle_types();
        assert!(!oracle_code.to_string().is_empty());
    }
}
