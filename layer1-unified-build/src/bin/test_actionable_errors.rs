use std::fs;
use syn::parse_file;
use unified_build::actionable_errors::{ActionableErrorGenerator, print_actionable_error};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test_file = std::env::args().nth(1)
        .unwrap_or_else(|| "../rust/compiler/rustc_hir_analysis/src/lib.rs".to_string());
    
    println!("🧪 TESTING ACTIONABLE ERRORS");
    println!("📁 File: {}", test_file);
    println!("{}", "═".repeat(60));
    
    let content = fs::read_to_string(&test_file)?;
    let error_generator = ActionableErrorGenerator::new();
    
    // Apply a transformation that will definitely break things
    let transformed = content.replace("use ", "broken_use ");
    
    // Test syn parsing
    match parse_file(&transformed) {
        Ok(_) => {
            println!("✅ File parses successfully after transformation");
        },
        Err(e) => {
            println!("❌ Syn parse error detected!");
            let actionable_error = error_generator.generate_actionable_error(&test_file, &e.to_string());
            print_actionable_error(&actionable_error);
            
            println!("\n🔧 DEMONSTRATION: Quick fix would be:");
            println!("   ./quick_fix.sh attributes");
            println!("   # This would fix: # [attr] → #[attr]");
        }
    }
    
    Ok(())
}
