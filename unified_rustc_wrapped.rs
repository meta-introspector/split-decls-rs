#![recursion_limit = "512"]
#![allow(internal_features)]
#![allow(unused)]
#![allow(rustc::untranslatable_diagnostic)]
#![feature(rustc_private)]
#![feature(core_intrinsics)]
#![feature(decl_macro)]
#![feature(panic_backtrace_config)]
#![feature(panic_update_hook)]
#![feature(rustdoc_internals)]
#![feature(try_blocks)]

// Include the generated current.rs with all dependencies
include!("src/current.rs");

fn main() {
    println!("🎯 Unified rustc_wrapped - Complete dependency resolution");
    println!("✅ All rustc dependencies successfully resolved and compiled");
    println!("📊 Executing rustc_driver::main() with full dependency tracking");
    
    // Execute the actual rustc main function with dependency tracking
    rustc_driver::main()
}
