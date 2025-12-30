// Generated macro for impl_specific_lit_to_lit (macro)
macro_rules! Depcrate_implsimpl_specific_lit_to_lit {
() => {
// Module: crate::impls
// Provides: {"impl_specific_lit_to_lit"}
// Dependencies: {}
macro_rules ! impl_specific_lit_to_lit { ($ ty : ty , $ variant : ident) => { impl < B : crate :: Buffer > From <$ ty > for Literal < B > { fn from (src : $ ty) -> Self { Literal ::$ variant (src) } } } ; }
};
}
