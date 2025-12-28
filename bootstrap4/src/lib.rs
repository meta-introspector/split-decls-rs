use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

// Telemetry data structure
#[derive(Debug, Clone)]
pub struct TelemetryData {
    pub function_name: String,
    pub start_time: SystemTime,
    pub duration_ms: u128,
    pub input_hash: String,
    pub output_hash: String,
    pub success: bool,
    pub call_id: u64,
}

static mut CALL_COUNTER: u64 = 0;
static mut TELEMETRY_LOG: Vec<TelemetryData> = Vec::new();

// Macro to wrap extracted functions with telemetry
macro_rules! telemetry_wrap {
    ($func_name:expr, $func_call:expr) => {{
        let start = Instant::now();
        let start_time = SystemTime::now();
        
        unsafe {
            CALL_COUNTER += 1;
        }
        let call_id = unsafe { CALL_COUNTER };
        
        println!("📊 [TELEMETRY] Starting {}, Call ID: {}", $func_name, call_id);
        
        let result = $func_call;
        
        let duration = start.elapsed();
        let success = result.is_ok();
        
        let telemetry = TelemetryData {
            function_name: $func_name.to_string(),
            start_time,
            duration_ms: duration.as_millis(),
            input_hash: "input_hash".to_string(), // Simplified for demo
            output_hash: "output_hash".to_string(), // Simplified for demo
            success,
            call_id,
        };
        
        unsafe {
            TELEMETRY_LOG.push(telemetry.clone());
        }
        
        println!("📊 [TELEMETRY] Completed {}, Duration: {}ms, Success: {}", 
                $func_name, duration.as_millis(), success);
        
        result
    }};
}

// Include the extracted function
include!("../../output2/wrapped-split-decls-rs/src/decls/wrapped_split_decls_rs_decls_paths_setup_crate_paths.rs");

/// Bootstrap4: Telemetry-wrapped execution of extracted functions
pub struct Bootstrap4Executor {
    audit_log: Vec<String>,
}

impl Bootstrap4Executor {
    pub fn new() -> Self {
        Self {
            audit_log: Vec::new(),
        }
    }

    /// Execute functions with telemetry wrapping
    pub fn execute_with_telemetry(&mut self, crate_path: &Path) -> Result<()> {
        self.log("🚀 Starting telemetry-wrapped execution");
        
        // Call the extracted function with telemetry wrapping
        let paths = telemetry_wrap!(
            "setup_crate_paths",
            setup_crate_paths(crate_path)
        )?;
        
        self.log(&format!("✅ Function executed with telemetry: {}", paths.crate_name));
        
        // Call it multiple times to show telemetry accumulation
        for i in 1..=3 {
            let _paths = telemetry_wrap!(
                "setup_crate_paths",
                setup_crate_paths(crate_path)
            )?;
            self.log(&format!("🔄 Telemetry call {}: completed", i));
        }
        
        self.show_telemetry_report();
        
        Ok(())
    }

    fn show_telemetry_report(&mut self) {
        self.log("📊 TELEMETRY REPORT:");
        
        unsafe {
            for (i, telemetry) in TELEMETRY_LOG.iter().enumerate() {
                self.log(&format!(
                    "  Call {}: {} (ID: {}) - {}ms - Success: {}",
                    i + 1,
                    telemetry.function_name,
                    telemetry.call_id,
                    telemetry.duration_ms,
                    telemetry.success
                ));
            }
            
            let total_calls = TELEMETRY_LOG.len();
            let total_duration: u128 = TELEMETRY_LOG.iter().map(|t| t.duration_ms).sum();
            let success_rate = TELEMETRY_LOG.iter().filter(|t| t.success).count() as f64 / total_calls as f64 * 100.0;
            
            self.log(&format!("📈 SUMMARY: {} calls, {}ms total, {:.1}% success rate", 
                            total_calls, total_duration, success_rate));
        }
    }

    fn log(&mut self, message: &str) {
        println!("[BOOTSTRAP4] {}", message);
        self.audit_log.push(message.to_string());
    }

    pub fn get_audit_log(&self) -> &[String] {
        &self.audit_log
    }
}

// Define the CratePaths struct
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
