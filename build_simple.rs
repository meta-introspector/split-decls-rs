// Simple build.rs that generates basic rustc includes
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Simple Build.rs - Generating basic rustc includes");
    
    // Create a simple rustc_includes.rs with basic structure
    let rustc_includes = r#"
// Auto-generated rustc includes
// Basic structure for compilation

pub mod rustc_complete {
    pub use alloc;
    pub use core;
    pub use std;
    
    // Basic type stubs
    pub mod ty {
        pub struct Ty<T>(pub T);
        pub struct TyCtxt<T>(pub T);
        pub struct TypeAndMut<T> { pub ty: T, pub mutbl: bool }
        
        pub mod layout {
            pub struct Layout;
            pub struct TyAndLayout<T> { pub ty: T, pub layout: Layout }
        }
    }
    
    pub mod def_id {
        pub struct DefId;
        pub struct LocalDefId;
        pub struct DefIndex;
        pub struct CrateNum;
    }
    
    pub mod mir {
        pub struct Body<T>(pub T);
        pub struct BasicBlock;
        pub struct Local;
        pub struct Place<T>(pub T);
    }
}
"#;
    
    let out_dir = std::env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("rustc_includes.rs");
    fs::write(&dest_path, rustc_includes)?;
    
    println!("✅ Generated rustc_includes.rs");
    Ok(())
}
"
