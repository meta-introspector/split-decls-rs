// Generated macro for impl_511 (impl)
macro_rules! Depcrate_utilsimpl_511 {
() => {
// Module: crate::utils
// Provides: {"impl_511"}
// Dependencies: {}
impl < 'ast > syn :: visit :: Visit < 'ast > for SearchSimpleTypeName { fn visit_path (& mut self , p : & 'ast syn :: Path) { if let Some (id) = p . get_ident () { self . 0 . insert (id . clone ()) ; } syn :: visit :: visit_path (self , p) } fn visit_lifetime (& mut self , i : & 'ast syn :: Lifetime) { self . 0 . insert (i . ident . clone ()) ; syn :: visit :: visit_lifetime (self , i) } }
};
}
