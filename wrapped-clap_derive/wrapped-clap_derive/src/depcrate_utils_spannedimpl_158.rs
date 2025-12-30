// Generated macro for impl_158 (impl)
macro_rules! Depcrate_utils_spannedimpl_158 {
() => {
// Module: crate::utils::spanned
// Provides: {"impl_158"}
// Dependencies: {}
impl From < LitStr > for Sp < String > { fn from (lit : LitStr) -> Self { Sp { val : lit . value () , span : lit . span () , } } }
};
}
