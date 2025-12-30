// Generated macro for mathml_text_integration_point (function)
macro_rules! Depcrate_tree_builder_tag_setsmathml_text_integration_point {
() => {
// Module: crate::tree_builder::tag_sets
// Provides: {"mathml_text_integration_point"}
// Dependencies: {}
pub (crate) fn mathml_text_integration_point (p : ExpandedName) -> bool { matches ! (p , expanded_name ! (mathml "mi") | expanded_name ! (mathml "mo") | expanded_name ! (mathml "mn") | expanded_name ! (mathml "ms") | expanded_name ! (mathml "mtext")) }
};
}
