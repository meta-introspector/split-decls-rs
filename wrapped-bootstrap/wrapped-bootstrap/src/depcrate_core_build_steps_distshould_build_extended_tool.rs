// Generated macro for should_build_extended_tool (function)
macro_rules! Depcrate_core_build_steps_distshould_build_extended_tool {
() => {
// Module: crate::core::build_steps::dist
// Provides: {"should_build_extended_tool"}
// Dependencies: {}
fn should_build_extended_tool (builder : & Builder < '_ > , tool : & str) -> bool { if ! builder . config . extended { return false ; } builder . config . tools . as_ref () . is_none_or (| tools | tools . contains (tool)) }
};
}
