// Generated macro for get_compiler_to_test (function)
macro_rules! Depcrate_core_build_steps_testget_compiler_to_test {
() => {
// Module: crate::core::build_steps::test
// Provides: {"get_compiler_to_test"}
// Dependencies: {}
fn get_compiler_to_test (builder : & Builder < '_ > , target : TargetSelection) -> Compiler { builder . compiler (builder . top_stage , target) }
};
}
