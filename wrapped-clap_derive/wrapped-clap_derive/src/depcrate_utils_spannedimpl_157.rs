// Generated macro for impl_157 (impl)
macro_rules! Depcrate_utils_spannedimpl_157 {
() => {
// Module: crate::utils::spanned
// Provides: {"impl_157"}
// Dependencies: {}
impl From < Ident > for Sp < String > { fn from (ident : Ident) -> Self { Sp { val : ident . to_string () , span : ident . span () , } } }
};
}
