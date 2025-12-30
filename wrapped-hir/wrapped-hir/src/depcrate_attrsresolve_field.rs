// Generated macro for resolve_field (function)
macro_rules! Depcrate_attrsresolve_field {
() => {
// Module: crate::attrs
// Provides: {"resolve_field"}
// Dependencies: {}
fn resolve_field (db : & dyn HirDatabase , def : VariantDef , name : Name , ns : Option < Namespace > ,) -> Option < DocLinkDef > { if let Some (Namespace :: Types | Namespace :: Macros) = ns { return None ; } def . fields (db) . into_iter () . find (| f | f . name (db) == name) . map (DocLinkDef :: Field) }
};
}
