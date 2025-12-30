// Generated macro for svg_html_integration_point (function)
macro_rules! Depcrate_tree_builder_tag_setssvg_html_integration_point {
() => {
// Module: crate::tree_builder::tag_sets
// Provides: {"svg_html_integration_point"}
// Dependencies: {}
# [doc = " https://html.spec.whatwg.org/multipage/#html-integration-point"] pub fn svg_html_integration_point (p : QualName) -> bool { matches ! (p , qualname ! (svg , "foreignObject") | qualname ! (svg , "desc") | qualname ! (svg , "title")) }
};
}
