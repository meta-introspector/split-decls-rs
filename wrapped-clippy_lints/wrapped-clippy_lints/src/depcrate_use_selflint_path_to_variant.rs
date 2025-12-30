// Generated macro for lint_path_to_variant (function)
macro_rules! Depcrate_use_selflint_path_to_variant {
() => {
// Module: crate::use_self
// Provides: {"lint_path_to_variant"}
// Dependencies: {}
fn lint_path_to_variant (cx : & LateContext < '_ > , path : & Path < '_ >) { if let [.. , self_seg , _variant] = path . segments { let span = path . span . with_hi (self_seg . args () . span_ext () . unwrap_or (self_seg . ident . span) . hi ()) ; span_lint (cx , span) ; } }
};
}
