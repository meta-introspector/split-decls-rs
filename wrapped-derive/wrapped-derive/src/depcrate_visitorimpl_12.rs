// Generated macro for impl_12 (impl)
macro_rules! Depcrate_visitorimpl_12 {
() => {
// Module: crate::visitor
// Provides: {"impl_12"}
// Dependencies: {}
impl < 'ast > Visit < 'ast > for TypeVisitor < '_ > { fn visit_lifetime (& mut self , lt : & 'ast Lifetime) { if lt . ident != "static" { self . found_lifetimes = true ; } visit_lifetime (self , lt) } fn visit_type_path (& mut self , ty : & 'ast TypePath) { if let Some (ident) = ty . path . get_ident () { if self . typarams . contains (ident) { self . found_typarams = true ; } } visit_type_path (self , ty) } }
};
}
