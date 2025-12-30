// Generated macro for test_evaluation (function)
macro_rules! Depcratetest_evaluation {
() => {
// Module: crate
// Provides: {"test_evaluation"}
// Dependencies: {}
fn test_evaluation (bin_name : & str) -> Result < () , Box < dyn std :: error :: Error > > { println ! ("🧪 Testing evaluation compilation for: {}" , bin_name) ; let eval_file = format ! ("../bootstrap3-incremental/{}_recursive_eval.rs" , bin_name) ; if ! std :: path :: Path :: new (& eval_file) . exists () { println ! ("❌ Evaluation file not found. Run recursive-deps first.") ; return Ok (()) ; } let test_dir = format ! ("../bootstrap3-incremental/test-{}" , bin_name) ; fs :: create_dir_all (& format ! ("{}/src" , test_dir)) ? ; let cargo_toml = format ! (r#"[package]
name = "test-{}"
version = "0.1.0"
edition = "2021"

[workspace]

[dependencies]
serde = {{ version = "1.0", features = ["derive"] }}
syn = {{ version = "2.0", features = ["full"] }}
quote = "1.0"
anyhow = "1.0"
"# , bin_name) ; fs :: write (format ! ("{}/Cargo.toml" , test_dir) , cargo_toml) ? ; let main_content = format ! (r#"// Test compilation of {} evaluation
use std::collections::HashMap;
use serde::{{Serialize, Deserialize}};

include!("../../{}_recursive_eval.rs");

fn main() -> Result<(), Box<dyn std::error::Error>> {{
    println!("🧪 Testing {} evaluation...");
    evaluate_{}()?;
    println!("✅ Evaluation compiled and ran successfully!");
    Ok(())
}}
"# , bin_name , bin_name , bin_name , bin_name) ; fs :: write (format ! ("{}/src/main.rs" , test_dir) , main_content) ? ; println ! ("🔨 Attempting compilation...") ; let output = Command :: new ("cargo") . args (& ["check"]) . current_dir (& test_dir) . output () ? ; if output . status . success () { println ! ("✅ SUCCESS: All dependencies compile in correct order!") ; println ! ("🚀 Attempting execution...") ; let run_output = Command :: new ("cargo") . args (& ["run"]) . current_dir (& test_dir) . output () ? ; if run_output . status . success () { println ! ("✅ EXECUTION SUCCESS!") ; println ! ("{}" , String :: from_utf8_lossy (& run_output . stdout)) ; } else { println ! ("⚠️  Compilation succeeded but execution failed:") ; println ! ("{}" , String :: from_utf8_lossy (& run_output . stderr)) ; } } else { println ! ("❌ COMPILATION FAILED:") ; println ! ("{}" , String :: from_utf8_lossy (& output . stderr)) ; let stderr = String :: from_utf8_lossy (& output . stderr) ; if stderr . contains ("cannot find") { println ! ("\n🔍 Dependency resolution issues detected.") ; println ! ("This suggests the topological ordering or token resolution needs improvement.") ; } } Ok (()) }
};
}
