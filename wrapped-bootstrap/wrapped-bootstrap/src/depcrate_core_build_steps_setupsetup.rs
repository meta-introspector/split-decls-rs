// Generated macro for setup (function)
macro_rules! Depcrate_core_build_steps_setupsetup {
() => {
// Module: crate::core::build_steps::setup
// Provides: {"setup"}
// Dependencies: {}
pub fn setup (config : & Config , profile : Profile) { let suggestions : & [& str] = match profile { Profile :: Compiler | Profile :: None => & ["check" , "build" , "test"] , Profile :: Tools => & ["check" , "build" , "test tests/rustdoc*" , "test src/tools/clippy" , "test src/tools/miri" , "test src/tools/rustfmt" ,] , Profile :: Library => & ["check" , "build" , "test library/std" , "doc"] , Profile :: Dist => & ["dist" , "build"] , } ; println ! () ; println ! ("To get started, try one of the following commands:") ; for cmd in suggestions { println ! ("- `x.py {cmd}`") ; } if profile != Profile :: Dist { println ! ("For more suggestions, see https://rustc-dev-guide.rust-lang.org/building/suggested.html") ; } if profile == Profile :: Tools { eprintln ! () ; eprintln ! ("NOTE: the `tools` profile sets up the `stage2` toolchain (use \
            `rustup toolchain link 'name' build/host/stage2` to use rustc)") } let path = & config . config . clone () . unwrap_or (PathBuf :: from ("bootstrap.toml")) ; setup_config_toml (path , profile , config) ; }
};
}
