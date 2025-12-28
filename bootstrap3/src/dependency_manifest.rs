// Auto-generated dependency manifest for output2 extracted functions
// This file maps each function to its required dependencies

use std::collections::HashMap;

pub struct DependencyManifest {
    pub functions: HashMap<String, Vec<String>>,
    pub types: HashMap<String, Vec<String>>,
    pub modules: HashMap<String, Vec<String>>,
}

impl DependencyManifest {
    pub fn new() -> Self {
        let mut manifest = Self {
            functions: HashMap::new(),
            types: HashMap::new(),
            modules: HashMap::new(),
        };
        
        // Auto-discovered dependencies from compilation errors
        manifest.functions.insert("run_wrapped_workspace_mode".to_string(), vec![
            "dep_to_toml_value_iter".to_string(),
            "generate_wrapped_workspace".to_string(),
            "build_script_composer::compose_build_script_from_parts".to_string(),
            "PatchConfig::load_from_file".to_string(),
        ]);
        
        manifest.functions.insert("generate_wrapped_workspace".to_string(), vec![
            "multi_crate::handle_multi_crate_wrapping".to_string(),
            "add_generated_header".to_string(),
            "format_generated_rust_files".to_string(),
        ]);
        
        manifest.types.insert("run_wrapped_workspace_mode".to_string(), vec![
            "eager_splitter::ModuleNotFoundReport".to_string(),
            "patch_config::PatchConfig".to_string(),
        ]);
        
        manifest
    }
    
    pub fn get_dependencies(&self, function_name: &str) -> Vec<String> {
        let mut deps = Vec::new();
        
        if let Some(func_deps) = self.functions.get(function_name) {
            deps.extend(func_deps.clone());
        }
        
        if let Some(type_deps) = self.types.get(function_name) {
            deps.extend(type_deps.clone());
        }
        
        deps
    }
}

// Macro to auto-include dependencies
#[macro_export]
macro_rules! auto_include_deps {
    ($func_name:literal) => {
        {
            let manifest = DependencyManifest::new();
            let deps = manifest.get_dependencies($func_name);
            println!("🔧 Auto-resolving {} dependencies for {}", deps.len(), $func_name);
            for dep in deps {
                println!("   📦 Need: {}", dep);
            }
        }
    };
}
