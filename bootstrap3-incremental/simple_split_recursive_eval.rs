// Recursive evaluation for simple_split
// Nix-like functional cache system

// Macro definitions
macro_rules! mkdeclfn {
    (fn $name:ident($($args:tt)*) -> Result<()> { $($body:tt)* }) => {
        pub fn $name($($args)*) -> Result<(), Box<dyn std::error::Error>> {
            println!("🔧 Calling function: {}", stringify!($name));
            $($body)*
        }
    };
    (fn $name:ident($($args:tt)*) -> $ret:ty { $($body:tt)* }) => {
        pub fn $name($($args)*) -> $ret {
            println!("🔧 Calling function: {}", stringify!($name));
            $($body)*
        }
    };
    (fn $name:ident($($args:tt)*) { $($body:tt)* }) => {
        pub fn $name($($args)*) {
            println!("🔧 Calling function: {}", stringify!($name));
            $($body)*
        }
    };
}

macro_rules! mkbin {
    (binary: $bin:expr, dependencies: [$($deps:expr),*], total_deps: $total:expr, cache_entries: $cache:expr) => {
        pub fn main() -> Result<(), Box<dyn std::error::Error>> {
            println!("🚀 Running binary: {}", $bin);
            println!("📦 Total dependencies: {}", $total);
            println!("💾 Cache entries: {}", $cache);
            $(
                println!("📄 Dependency: {}", $deps);
            )*
            evaluate_simple_split()?;
            Ok(())
        }
    };
}

// Dep: dep-e6c2032e2a86ab3d (hash: 42eb9df40dc969bb)
mod dep_mod_0 {
    use std :: collections :: HashMap ;

    use std :: path :: Path ;
    use std :: fs ;
    use std :: env ;
    
    mkdeclfn ! { fn main () -> Result < () > { 
        let args : Vec < String > = env :: args () . collect () ; 
        let crate_path = if args . len () > 1 { & args [1] } else { "." } ; 
        println ! ("🔍 Analyzing crate: {}" , crate_path) ; 
        let lib_rs = Path :: new (crate_path) . join ("src/lib.rs") ; 
        if ! lib_rs . exists () { 
            println ! ("❌ No src/lib.rs found") ; 
            return Ok (()) ; 
        } 
        println ! ("✅ Found lib.rs at: {:?}" , lib_rs) ; 
        Ok (()) 
    } }
}
pub use dep_mod_0::*;

mkdeclfn! { fn evaluate_simple_split() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Evaluating recursive dependencies...");
    // All dependencies are now available
    Ok(())
} }

mkbin! {
    binary: "simple_split",
    dependencies: [
        "../output2/wrapped-split-decls-rs/src/decls/simple_split/fn/7/main.rs"
    ],
    total_deps: 1,
    cache_entries: 7
}
