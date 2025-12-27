#![feature(stmt_expr_attributes)]

use std::collections::HashMap;
use std::path::Path;
use anyhow::Result;

/// Import all output2 declarations as callable macros
/// Creates a Lisp-like runtime system for code generation
#[derive(Debug)]
pub struct Output2MacroSystem {
    /// All available macros from output2 declarations
    pub macros: HashMap<String, MacroDeclaration>,
    /// Runtime interpreter for macro calls
    pub interpreter: LispInterpreter,
}

#[derive(Debug, Clone)]
pub struct MacroDeclaration {
    pub name: String,
    pub source_path: String,
    pub declaration_type: String, // "fn", "struct", "enum", "macro", etc.
    pub content: String,
    pub wrapper: Option<String>, // Our overlay wrapper
}

#[derive(Debug)]
pub struct LispInterpreter {
    pub environment: HashMap<String, MacroValue>,
}

#[derive(Debug, Clone)]
pub enum MacroValue {
    Function(String),
    Struct(String), 
    Enum(String),
    Macro(String),
    Generated(String), // Runtime generated code
}

impl Output2MacroSystem {
    /// Scan output2 and import all declarations as macros
    pub fn import_from_output2() -> Result<Self> {
        let mut macros = HashMap::new();
        let output2_path = Path::new("output2");
        
        // Scan all wrapped crates in output2
        for entry in #[syscall="read"]
    std::fs::read_dir(output2_path)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let crate_name = entry.file_name().to_string_lossy().to_string();
                if crate_name.starts_with("wrapped-") {
                    Self::import_crate_declarations(&mut macros, &entry.path())?;
                }
            }
        }
        
        println!("📦 Imported {} macro declarations from output2", macros.len());
        
        Ok(Output2MacroSystem {
            macros,
            interpreter: LispInterpreter::new(),
        })
    }
    
    /// Import all declarations from a single wrapped crate
    fn import_crate_declarations(macros: &mut HashMap<String, MacroDeclaration>, crate_path: &Path) -> Result<()> {
        let decls_path = crate_path.join("src/decls");
        if !decls_path.exists() {
            return Ok(());
        }
        
        for entry in #[syscall="read"]
    std::fs::read_dir(decls_path)? {
            let entry = entry?;
            if entry.path().extension().map_or(false, |ext| ext == "rs") {
                let file_name = entry.file_name().to_string_lossy().to_string();
                let content = #[syscall="read"]
    std::fs::read_to_string(entry.path())?;
                
                // Extract declaration info from filename and content
                let (decl_name, decl_type) = Self::parse_declaration_info(&file_name, &content);
                
                let macro_decl = MacroDeclaration {
                    name: decl_name.clone(),
                    source_path: entry.path().to_string_lossy().to_string(),
                    declaration_type: decl_type,
                    content,
                    wrapper: None, // Will be overlaid later
                };
                
                macros.insert(decl_name, macro_decl);
            }
        }
        
        Ok(())
    }
    
    /// Parse declaration name and type from filename and content
    fn parse_declaration_info(filename: &str, content: &str) -> (String, String) {
        // Extract from filename pattern: crate_decls_name.rs
        let parts: Vec<&str> = filename.trim_end_matches(".rs").split('_').collect();
        let decl_name = parts.last().map_or("unknown", |v| v).to_string();
        
        // Determine type from content
        let decl_type = if content.contains("pub fn ") {
            "function"
        } else if content.contains("pub struct ") {
            "struct"
        } else if content.contains("pub enum ") {
            "enum"
        } else if content.contains("macro_rules!") {
            "macro"
        } else {
            "unknown"
        };
        
        (decl_name, decl_type.to_string())
    }
    
    /// Overlay wrapper macros on imported declarations
    pub fn overlay_wrappers(&mut self) {
        for (name, decl) in &mut self.macros {
            let wrapper = match decl.declaration_type.as_str() {
                "function" => Some(format!("
                    macro_rules! call_{} {{
                        ($($args:expr),*) => {{
                            // Runtime wrapper for {}
                            println!(\"🔧 Calling function: {}\");
                            {}($($args),*)
                        }};
                    }}", name, name, name, name)),
                "struct" => Some(format!("
                    macro_rules! create_{} {{
                        ($($field:ident: $value:expr),*) => {{
                            println!(\"🏗️ Creating struct: {}\");
                            {} {{ $($field: $value),* }}
                        }};
                    }}", name, name, name)),
                "macro" => Some(format!("
                    macro_rules! invoke_{} {{
                        ($($args:tt)*) => {{
                            println!(\"🎭 Invoking macro: {}\");
                            {}!($($args)*)
                        }};
                    }}", name, name, name)),
                _ => None,
            };
            
            decl.wrapper = wrapper;
        }
        
        println!("🎭 Overlaid wrappers on {} declarations", self.macros.len());
    }
    
    /// Generate wrapped rustdoc with all callable macros
    pub fn generate_rustdoc(&self) -> Result<String> {
        let mut doc = String::new();
        
        doc.push_str("//! # Output2 Macro System - Lisp-like Runtime Code Generation\n");
        doc.push_str("//! \n");
        doc.push_str("//! All declarations from output2 wrapped crates, callable as macros.\n");
        doc.push_str("//! Runtime interpretation system for dynamic code generation.\n\n");
        
        // Group by type
        let mut functions = Vec::new();
        let mut structs = Vec::new();
        let mut macros = Vec::new();
        
        for (name, decl) in &self.macros {
            match decl.declaration_type.as_str() {
                "function" => functions.push((name, decl)),
                "struct" => structs.push((name, decl)),
                "macro" => macros.push((name, decl)),
                _ => {}
            }
        }
        
        // Generate documentation sections
        if !functions.is_empty() {
            doc.push_str("## 🔧 Callable Functions\n\n");
            for (name, decl) in functions {
                if let Some(wrapper) = &decl.wrapper {
                    doc.push_str(&format!("### `call_{}!()`\n", name));
                    doc.push_str(&format!("Source: `{}`\n\n", decl.source_path));
                    doc.push_str("```rust\n");
                    doc.push_str(wrapper);
                    doc.push_str("\n```\n\n");
                }
            }
        }
        
        if !structs.is_empty() {
            doc.push_str("## 🏗️ Creatable Structs\n\n");
            for (name, decl) in structs {
                if let Some(wrapper) = &decl.wrapper {
                    doc.push_str(&format!("### `create_{}!()`\n", name));
                    doc.push_str(&format!("Source: `{}`\n\n", decl.source_path));
                    doc.push_str("```rust\n");
                    doc.push_str(wrapper);
                    doc.push_str("\n```\n\n");
                }
            }
        }
        
        if !macros.is_empty() {
            doc.push_str("## 🎭 Invokable Macros\n\n");
            for (name, decl) in macros {
                if let Some(wrapper) = &decl.wrapper {
                    doc.push_str(&format!("### `invoke_{}!()`\n", name));
                    doc.push_str(&format!("Source: `{}`\n\n", decl.source_path));
                    doc.push_str("```rust\n");
                    doc.push_str(wrapper);
                    doc.push_str("\n```\n\n");
                }
            }
        }
        
        Ok(doc)
    }
}

impl LispInterpreter {
    pub fn new() -> Self {
        LispInterpreter {
            environment: HashMap::new(),
        }
    }
    
    /// Interpret macro calls at runtime (Lisp-like evaluation)
    pub fn eval(&mut self, expression: &str) -> Result<MacroValue> {
        // Simple S-expression parser for macro calls
        if expression.starts_with("(call_") {
            // Parse function call: (call_function_name arg1 arg2)
            let parts: Vec<&str> = expression.trim_matches(|c| c == '(' || c == ')').split_whitespace().collect();
            let func_name = parts[0].strip_prefix("call_").unwrap_or("unknown");
            let args = &parts[1..];
            
            println!("🧠 Lisp eval: Calling {} with args: {:?}", func_name, args);
            Ok(MacroValue::Generated(format!("{}({});", func_name, args.join(", "))))
        } else if expression.starts_with("(create_") {
            // Parse struct creation: (create_struct_name field1 value1 field2 value2)
            let parts: Vec<&str> = expression.trim_matches(|c| c == '(' || c == ')').split_whitespace().collect();
            let struct_name = parts[0].strip_prefix("create_").unwrap_or("unknown");
            
            println!("🧠 Lisp eval: Creating struct {}", struct_name);
            Ok(MacroValue::Generated(format!("{} {{ /* fields */ }}", struct_name)))
        } else {
            Ok(MacroValue::Generated(format!("/* Unknown expression: {} */", expression)))
        }
    }
    
    /// Generate code from interpreted expressions
    pub fn generate_code(&self, expressions: &[String]) -> Result<String> {
        let mut code = String::new();
        code.push_str("// Generated code from Lisp-like macro interpretation\n\n");
        
        for expr in expressions {
            // This would contain the actual generated code
            code.push_str(&format!("// Expression: {}\n", expr));
        }
        
        Ok(code)
    }
}
