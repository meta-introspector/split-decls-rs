// Generated macro for impl_ieee_pi (macro)
macro_rules! Depcrate_mathimpl_ieee_pi {
() => {
// Module: crate::math
// Provides: {"impl_ieee_pi"}
// Dependencies: {}
macro_rules ! impl_ieee_pi { ($ float_ty : ident , $ semantic : ty) => { impl IeeeExt for IeeeFloat <$ semantic > { # [inline] fn pi () -> Self { Self :: from_bits ($ float_ty :: consts :: PI . to_bits () . into ()) } } } ; }
};
}
