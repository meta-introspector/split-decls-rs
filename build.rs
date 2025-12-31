use std::fs;
use anyhow::Result;

macro_rules! mkuniverse {
    (
        blockchain: $blockchain:literal,
        compiler: $compiler:literal,
        test: $test:literal,
        lib: $lib:literal
    ) => {
        fn main() -> Result<()> {
            println!("🌟 GENERATING FROM MACRO PARAMETERS");
            
            fs::create_dir_all("src")?;
            fs::create_dir_all("src/bin")?;
            
            fs::write("src/blockchain_macros.rs", $blockchain)?;
            fs::write("src/compiler_macros.rs", $compiler)?;
            fs::write("src/bin/test_universe.rs", $test)?;
            fs::write("src/lib.rs", $lib)?;
            
            println!("🧬 Generated complete system from macro parameters!");
            Ok(())
        }
    };
}

mkuniverse! {
    blockchain: "pub struct Contract { pub address: &'static str }",
    compiler: "pub struct RustCompiler { pub version: &'static str }",
    test: "fn main() { println!(\"Test from macro param!\"); }",
    lib: "// Generated lib from macro parameter"
}
