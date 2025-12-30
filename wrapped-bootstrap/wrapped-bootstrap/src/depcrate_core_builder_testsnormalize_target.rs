// Generated macro for normalize_target (function)
macro_rules! Depcrate_core_builder_testsnormalize_target {
() => {
// Module: crate::core::builder::tests
// Provides: {"normalize_target"}
// Dependencies: {}
fn normalize_target (target : TargetSelection , config : & RenderConfig) -> String { let mut target = target . to_string () ; if config . normalize_host { target = target . replace (& host_target () , "host") ; } target . replace (TEST_TRIPLE_1 , "target1") . replace (TEST_TRIPLE_2 , "target2") }
};
}
