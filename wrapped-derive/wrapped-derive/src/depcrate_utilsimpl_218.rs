// Generated macro for impl_218 (impl)
macro_rules! Depcrate_utilsimpl_218 {
() => {
// Module: crate::utils
// Provides: {"impl_218"}
// Dependencies: {}
impl VisitMut for RemoveLifetime { fn visit_lifetime_mut (& mut self , i : & mut Lifetime) { i . ident = Ident :: new ("_" , Span :: call_site ()) ; visit_mut :: visit_lifetime_mut (self , i) ; } }
};
}
