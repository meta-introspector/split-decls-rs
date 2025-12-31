use std::fs;
use anyhow::Result;

// Standalone content functions
fn get_blockchain_content() -> &'static str {
    "pub struct Contract { pub address: &'static str }"
}

fn get_compiler_content() -> &'static str {
    "pub struct RustCompiler { pub version: &'static str }"
}

fn get_test_content() -> &'static str {
    "fn main() { println!(\"Test from clean functions!\"); }"
}

fn get_lib_content() -> &'static str {
    "pub mod blockchain_macros;\npub mod compiler_macros;"
}

// Macro that uses the standalone functions
macro_rules! mkuniverse {
    (
        blockchain: $blockchain_fn:ident,
        compiler: $compiler_fn:ident,
        test: $test_fn:ident,
        lib: $lib_fn:ident
    ) => {
        fn main() -> Result<()> {
            println!("🌌 GENERATING FROM CLEAN FUNCTIONS");
            
            fs::create_dir_all("src")?;
            fs::create_dir_all("src/bin")?;
            
            fs::write("src/blockchain_macros.rs", $blockchain_fn())?;
            fs::write("src/compiler_macros.rs", $compiler_fn())?;
            fs::write("src/bin/test_universe.rs", $test_fn())?;
            fs::write("src/lib.rs", $lib_fn())?;
            
            println!("🧬 Generated from clean standalone functions!");
            Ok(())
        }
    };
}

// Generate universe using clean functions
mkuniverse! {
    blockchain: get_blockchain_content,
    compiler: get_compiler_content,
    test: get_test_content,
    lib: get_lib_content
}
