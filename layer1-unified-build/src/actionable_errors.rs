use std::collections::HashMap;

pub struct ActionableError {
    pub file: String,
    pub error_type: ErrorType,
    pub line: Option<usize>,
    pub quick_fix: String,
    pub command: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ErrorType {
    SynParseError(String),
    MissingImport(String),
    UnknownMacro(String),
    AttributeSpacing,
    TypeNotFound(String),
    ModuleNotFound(String),
}

pub struct ActionableErrorGenerator {
    common_fixes: HashMap<String, String>,
}

impl ActionableErrorGenerator {
    pub fn new() -> Self {
        let mut common_fixes = HashMap::new();
        
        // Common syn parse errors and their fixes
        common_fixes.insert(
            "expected square brackets".to_string(),
            "Fix attribute spacing: # [attr] → #[attr]".to_string()
        );
        
        common_fixes.insert(
            "macro not found".to_string(),
            "Add macro definition to transform_safe.rs".to_string()
        );
        
        common_fixes.insert(
            "unresolved import".to_string(),
            "Add import to transform_imports.rs".to_string()
        );
        
        Self { common_fixes }
    }
    
    pub fn generate_actionable_error(&self, file: &str, error: &str) -> ActionableError {
        let error_type = self.classify_error(error);
        let quick_fix = self.generate_quick_fix(&error_type, error);
        let command = self.generate_fix_command(&error_type, file);
        
        ActionableError {
            file: file.to_string(),
            error_type,
            line: self.extract_line_number(error),
            quick_fix,
            command,
        }
    }
    
    fn classify_error(&self, error: &str) -> ErrorType {
        if error.contains("expected square brackets") {
            ErrorType::AttributeSpacing
        } else if error.contains("macro") && error.contains("not found") {
            let macro_name = self.extract_macro_name(error);
            ErrorType::UnknownMacro(macro_name)
        } else if error.contains("unresolved import") {
            let import_name = self.extract_import_name(error);
            ErrorType::MissingImport(import_name)
        } else if error.contains("cannot find type") {
            let type_name = self.extract_type_name(error);
            ErrorType::TypeNotFound(type_name)
        } else {
            ErrorType::SynParseError(error.to_string())
        }
    }
    
    fn generate_quick_fix(&self, error_type: &ErrorType, error: &str) -> String {
        match error_type {
            ErrorType::AttributeSpacing => {
                "Run: Fix attribute spacing in transform_strings.rs".to_string()
            },
            ErrorType::UnknownMacro(macro_name) => {
                format!("Add macro definition: macro_rules! {} {{ ... }}", macro_name)
            },
            ErrorType::MissingImport(import) => {
                format!("Add import: use {};", import)
            },
            ErrorType::TypeNotFound(type_name) => {
                format!("Add type stub: pub struct {};", type_name)
            },
            ErrorType::SynParseError(_) => {
                self.common_fixes.get(error)
                    .cloned()
                    .unwrap_or_else(|| "Manual investigation needed".to_string())
            },
            _ => "Check transformation pipeline".to_string(),
        }
    }
    
    fn generate_fix_command(&self, error_type: &ErrorType, _file: &str) -> Option<String> {
        match error_type {
            ErrorType::AttributeSpacing => {
                Some("cd layer1-unified-build && cargo run --bin fix_attributes".to_string())
            },
            ErrorType::UnknownMacro(macro_name) => {
                Some(format!("echo 'macro_rules! {} {{ ($($tt:tt)*) => {{ $($tt)* }}; }}' >> src/macro_stubs.rs", macro_name))
            },
            ErrorType::MissingImport(import) => {
                Some(format!("echo 'pub use {};' >> src/import_stubs.rs", import))
            },
            _ => None,
        }
    }
    
    fn extract_line_number(&self, error: &str) -> Option<usize> {
        // Extract line number from syn error messages
        if let Some(start) = error.find("line ") {
            let line_part = &error[start + 5..];
            if let Some(end) = line_part.find(|c: char| !c.is_ascii_digit()) {
                line_part[..end].parse().ok()
            } else {
                line_part.parse().ok()
            }
        } else {
            None
        }
    }
    
    fn extract_macro_name(&self, error: &str) -> String {
        // Extract macro name from "macro `name` not found"
        if let Some(start) = error.find('`') {
            if let Some(end) = error[start + 1..].find('`') {
                return error[start + 1..start + 1 + end].to_string();
            }
        }
        "unknown_macro".to_string()
    }
    
    fn extract_import_name(&self, error: &str) -> String {
        // Extract import from "unresolved import `path`"
        if let Some(start) = error.find('`') {
            if let Some(end) = error[start + 1..].find('`') {
                return error[start + 1..start + 1 + end].to_string();
            }
        }
        "unknown_import".to_string()
    }
    
    fn extract_type_name(&self, error: &str) -> String {
        // Extract type from "cannot find type `TypeName`"
        if let Some(start) = error.find('`') {
            if let Some(end) = error[start + 1..].find('`') {
                return error[start + 1..start + 1 + end].to_string();
            }
        }
        "UnknownType".to_string()
    }
}

pub fn print_actionable_error(error: &ActionableError) {
    println!("\n🚨 ACTIONABLE ERROR");
    println!("📁 File: {}", error.file);
    if let Some(line) = error.line {
        println!("📍 Line: {}", line);
    }
    println!("🔧 Quick Fix: {}", error.quick_fix);
    
    if let Some(command) = &error.command {
        println!("⚡ Run This: {}", command);
    }
    
    println!("{}", "─".repeat(60));
}
