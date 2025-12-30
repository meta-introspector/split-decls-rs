// Generated macro for mathml_text_integration_point (function)
macro_rules! Depcrate_tree_builder_tag_setsmathml_text_integration_point {
() => {
// Module: crate::tree_builder::tag_sets
// Provides: {"mathml_text_integration_point"}
// Dependencies: {}
pub fn mathml_text_integration_point (p : QualName) -> bool { matches ! (p , qualname ! (mathml , "mi") | qualname ! (mathml , "mo") | qualname ! (mathml , "mn") | qualname ! (mathml , "ms") | qualname ! (mathml , "mtext")) }
};
}
