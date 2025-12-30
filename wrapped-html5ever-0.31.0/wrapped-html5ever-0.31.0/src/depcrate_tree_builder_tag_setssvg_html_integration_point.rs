// Generated macro for svg_html_integration_point (function)
macro_rules! Depcrate_tree_builder_tag_setssvg_html_integration_point {
() => {
// Module: crate::tree_builder::tag_sets
// Provides: {"svg_html_integration_point"}
// Dependencies: {}
# [doc = " https://html.spec.whatwg.org/multipage/#html-integration-point"] pub (crate) fn svg_html_integration_point (p : ExpandedName) -> bool { matches ! (p , expanded_name ! (svg "foreignObject") | expanded_name ! (svg "desc") | expanded_name ! (svg "title")) }
};
}
