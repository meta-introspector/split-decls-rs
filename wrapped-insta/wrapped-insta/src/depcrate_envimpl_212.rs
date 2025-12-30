// Generated macro for impl_212 (impl)
macro_rules! Depcrate_envimpl_212 {
() => {
// Module: crate::env
// Provides: {"impl_212"}
// Dependencies: {}
# [cfg (feature = "_cargo_insta_internal")] impl TestRunner { # [doc = " Fall back to `cargo test` if `cargo nextest` isn't installed and"] # [doc = " `test_runner_fallback` is true"] pub fn resolve_fallback (& self , test_runner_fallback : bool) -> & TestRunner { use crate :: utils :: get_cargo ; if self == & TestRunner :: Nextest && test_runner_fallback && std :: process :: Command :: new (get_cargo ()) . arg ("nextest") . arg ("--version") . output () . map (| output | ! output . status . success ()) . unwrap_or (true) { & TestRunner :: Auto } else { self } } }
};
}
