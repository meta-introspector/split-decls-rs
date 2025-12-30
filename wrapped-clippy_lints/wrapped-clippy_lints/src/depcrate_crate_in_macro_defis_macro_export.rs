// Generated macro for is_macro_export (function)
macro_rules! Depcrate_crate_in_macro_defis_macro_export {
() => {
// Module: crate::crate_in_macro_def
// Provides: {"is_macro_export"}
// Dependencies: {}
fn is_macro_export (attr : & Attribute) -> bool { if let AttrKind :: Normal (normal) = & attr . kind && let [segment] = normal . item . path . segments . as_slice () { segment . ident . name == sym :: macro_export } else { false } }
};
}
