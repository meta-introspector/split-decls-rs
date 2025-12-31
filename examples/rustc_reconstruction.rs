//! Example: Reconstructing rustc using generated function macros
//! 
//! This demonstrates how to use the auto-generated macros to build
//! a complete rustc compiler from individual function implementations.

// Include the generated macros
include!("../rustc_function_macros.rs");

// Example: Implement a core function
core_int_log10_i128! {
    fn core_int_log10_i128(value: i128) -> u32 {
        if value == 0 { return 0; }
        value.abs().ilog10()
    }
}

// Example: Implement rustc components
rustc_ast! {
    pub mod ast {
        pub struct Expr;
        pub struct Item;
        pub fn parse_expr() -> Expr { Expr }
    }
}

rustc_hir! {
    pub mod hir {
        pub struct Body;
        pub struct Expr;
        pub fn lower_expr(ast_expr: crate::ast::Expr) -> Expr { Expr }
    }
}

rustc_mir_build! {
    pub mod mir {
        pub struct Body;
        pub fn build_mir(hir_body: crate::hir::Body) -> Body { Body }
    }
}

rustc_codegen_llvm! {
    pub mod codegen {
        pub fn compile_to_llvm(mir_body: crate::mir::Body) -> String {
            "LLVM IR code".to_string()
        }
    }
}

// Example: Main rustc function using macros
fn main() {
    println!("🦀 Reconstructed rustc using function macros!");
    
    // Demonstrate the compilation pipeline
    let ast_expr = ast::parse_expr();
    let hir_expr = hir::lower_expr(ast_expr);
    let mir_body = mir::build_mir(hir::Body);
    let llvm_ir = codegen::compile_to_llvm(mir_body);
    
    println!("✅ Compiled to: {}", llvm_ir);
    
    // Test core function
    let log_result = core_int_log10_i128(12345);
    println!("📊 log10(12345) = {}", log_result);
}

// Uncomment to compose entire rustc (will fail until all functions implemented)
// compose_rustc!();
