// Generated macro for maybe_name_by_expr (function)
macro_rules! Depcrate_transmute_missing_transmute_annotationsmaybe_name_by_expr {
() => {
// Module: crate::transmute::missing_transmute_annotations
// Provides: {"maybe_name_by_expr"}
// Dependencies: {}
fn maybe_name_by_expr < 'a > (sess : & impl HasSession , span : Span , default : & 'a str) -> Cow < 'a , str > { span . with_source_text (sess , | name | { (name . len () + 9 < default . len ()) . then_some (format ! ("`{name}`'s type") . into ()) }) . flatten () . unwrap_or (default . into ()) }
};
}
