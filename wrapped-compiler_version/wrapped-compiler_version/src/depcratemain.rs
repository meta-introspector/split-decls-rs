// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { Command :: new (env ! ("CARGO")) . args (& ["rustc" , "--quiet" , "--" , "--version"]) . status () . unwrap () ; println ! () ; Command :: new (env ! ("CARGO")) . args (& ["--version"]) . status () . unwrap () ; println ! () ; let compiler_path = env ! ("COMPILER_PATH") ; let mut compiler_command = Command :: new (compiler_path) ; if ! cfg ! (target_env = "msvc") { compiler_command . arg ("--version") ; } let _ = compiler_command . status () . unwrap () ; }
};
}
