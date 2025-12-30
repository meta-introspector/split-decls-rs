// Generated macro for impl_937 (impl)
macro_rules! Depcrate_core_builderimpl_937 {
() => {
// Module: crate::core::builder
// Provides: {"impl_937"}
// Dependencies: {}
impl Kind { pub fn as_str (& self) -> & 'static str { match self { Kind :: Build => "build" , Kind :: Check => "check" , Kind :: Clippy => "clippy" , Kind :: Fix => "fix" , Kind :: Format => "fmt" , Kind :: Test => "test" , Kind :: Miri => "miri" , Kind :: MiriSetup => panic ! ("`as_str` is not supported for `Kind::MiriSetup`.") , Kind :: MiriTest => panic ! ("`as_str` is not supported for `Kind::MiriTest`.") , Kind :: Bench => "bench" , Kind :: Doc => "doc" , Kind :: Clean => "clean" , Kind :: Dist => "dist" , Kind :: Install => "install" , Kind :: Run => "run" , Kind :: Setup => "setup" , Kind :: Vendor => "vendor" , Kind :: Perf => "perf" , } } pub fn description (& self) -> String { match self { Kind :: Test => "Testing" , Kind :: Bench => "Benchmarking" , Kind :: Doc => "Documenting" , Kind :: Run => "Running" , Kind :: Clippy => "Linting" , Kind :: Perf => "Profiling & benchmarking" , _ => { let title_letter = self . as_str () [0 .. 1] . to_ascii_uppercase () ; return format ! ("{title_letter}{}ing" , & self . as_str () [1 ..]) ; } } . to_owned () } }
};
}
