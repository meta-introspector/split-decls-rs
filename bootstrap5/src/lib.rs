use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use std::time::{Instant, SystemTime};
use std::collections::HashMap;

// Call stack tracking
static mut CALL_STACK: Vec<String> = Vec::new();
static mut CALL_COUNTER: u64 = 0;
static mut CALL_TREE: Vec<CallTrace> = Vec::new();

#[derive(Debug, Clone)]
pub struct CallTrace {
    pub call_id: u64,
    pub function_name: String,
    pub parent_id: Option<u64>,
    pub depth: usize,
    pub duration_ms: u128,
    pub success: bool,
}

// Enhanced macro for call tracing
macro_rules! trace_call {
    ($func_name:expr, $func_call:expr) => {{
        let start = Instant::now();
        
        unsafe {
            CALL_COUNTER += 1;
            let current_id = CALL_COUNTER;
            let parent_id = if CALL_STACK.is_empty() { None } else { Some(CALL_COUNTER - 1) };
            let depth = CALL_STACK.len();
            
            CALL_STACK.push($func_name.to_string());
            
            println!("{}📞 [TRACE] {} (ID: {}, Depth: {})", 
                    "  ".repeat(depth), $func_name, current_id, depth);
            
            let result = $func_call;
            
            let duration = start.elapsed();
            let success = result.is_ok();
            
            CALL_TREE.push(CallTrace {
                call_id: current_id,
                function_name: $func_name.to_string(),
                parent_id,
                depth,
                duration_ms: duration.as_millis(),
                success,
            });
            
            CALL_STACK.pop();
            
            println!("{}✅ [TRACE] {} completed ({}ms)", 
                    "  ".repeat(depth), $func_name, duration.as_millis());
            
            result
        }
    }};
}

// Include multiple extracted functions
include!("../../output2/wrapped-split-decls-rs/src/decls/wrapped_split_decls_rs_decls_paths_setup_crate_paths.rs");
include!("../../output2/wrapped-split-decls-rs/src/decls/wrapped_split_decls_rs_decls_generate_wrapped_crate_generate_wrapped_crate.rs");

// Wrapper functions with tracing
pub fn traced_setup_crate_paths(crate_path: &Path) -> Result<CratePaths> {
    trace_call!("setup_crate_paths", setup_crate_paths(crate_path))
}

pub fn traced_generate_wrapped_crate(
    wrapped_workspace_root: &Path,
    original_crate_name: &str,
    original_crate_path: &Path,
    global_config: &SplitDeclsConfig,
    patch_config: &PatchConfig,
    dry_run: bool,
    cargo_only: bool,
) -> Result<Vec<ModuleNotFoundReport>> {
    trace_call!(
        "generate_wrapped_crate",
        generate_wrapped_crate(
            wrapped_workspace_root,
            original_crate_name,
            original_crate_path,
            global_config,
            patch_config,
            dry_run,
            cargo_only
        )
    )
}

/// Bootstrap5: Call tracing of extracted functions
pub struct Bootstrap5Executor {
    audit_log: Vec<String>,
}

impl Bootstrap5Executor {
    pub fn new() -> Self {
        Self {
            audit_log: Vec::new(),
        }
    }

    /// Execute a big function and trace all its calls
    pub fn execute_with_call_tracing(&mut self, crate_path: &Path) -> Result<()> {
        self.log("🚀 Starting call tracing execution");
        
        // Clear previous traces
        unsafe {
            CALL_STACK.clear();
            CALL_TREE.clear();
            CALL_COUNTER = 0;
        }
        
        // Execute the big function with tracing
        self.log("📋 Executing traced_setup_crate_paths");
        let _paths = traced_setup_crate_paths(crate_path)?;
        
        // Simulate calling a bigger function that calls multiple sub-functions
        self.log("📋 Executing multiple traced functions");
        for i in 1..=3 {
            let _paths = trace_call!(
                &format!("setup_crate_paths_call_{}", i),
                setup_crate_paths(crate_path)
            )?;
        }
        
        self.show_call_tree();
        
        Ok(())
    }

    fn show_call_tree(&mut self) {
        self.log("🌳 CALL TREE ANALYSIS:");
        
        unsafe {
            for trace in &CALL_TREE {
                let indent = "  ".repeat(trace.depth);
                let parent_info = match trace.parent_id {
                    Some(pid) => format!(" (parent: {})", pid),
                    None => " (root)".to_string(),
                };
                
                self.log(&format!(
                    "{}├─ {} (ID: {}{}) - {}ms - {}",
                    indent,
                    trace.function_name,
                    trace.call_id,
                    parent_info,
                    trace.duration_ms,
                    if trace.success { "✅" } else { "❌" }
                ));
            }
            
            let total_calls = CALL_TREE.len();
            let total_duration: u128 = CALL_TREE.iter().map(|t| t.duration_ms).sum();
            let max_depth = CALL_TREE.iter().map(|t| t.depth).max().unwrap_or(0);
            
            self.log(&format!("📊 CALL STATISTICS:"));
            self.log(&format!("  Total calls: {}", total_calls));
            self.log(&format!("  Total duration: {}ms", total_duration));
            self.log(&format!("  Max call depth: {}", max_depth));
        }
    }

    fn log(&mut self, message: &str) {
        println!("[BOOTSTRAP5] {}", message);
        self.audit_log.push(message.to_string());
    }

    pub fn get_audit_log(&self) -> &[String] {
        &self.audit_log
    }
}

// Required type definitions (simplified)
#[derive(Debug, Clone)]
pub struct CratePaths {
    pub crate_path: PathBuf,
    pub crate_name: String,
    pub lib_rs_path: PathBuf,
    pub build_rs_path: PathBuf,
    pub cargo_toml_path: PathBuf,
    pub decls_output_dir: PathBuf,
    pub target_config_path: PathBuf,
    pub output_crate_path: PathBuf,
}

#[derive(Debug, Clone, Default)]
pub struct SplitDeclsConfig {
    pub patches: Option<HashMap<String, Vec<String>>>,
    pub string_replacements: Option<Vec<StringReplacement>>,
    pub custom_prelude_overlay: Option<String>,
    pub active_overlay_modules: Vec<String>,
    pub crates_io_patches: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct StringReplacement {
    pub old: String,
    pub new: String,
}

#[derive(Debug, Clone, Default)]
pub struct PatchConfig;

#[derive(Debug, Clone)]
pub struct ModuleNotFoundReport {
    pub module_name: String,
    pub error: String,
}
