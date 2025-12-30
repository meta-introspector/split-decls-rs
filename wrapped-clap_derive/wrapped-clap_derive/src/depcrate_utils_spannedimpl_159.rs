// Generated macro for impl_159 (impl)
macro_rules! Depcrate_utils_spannedimpl_159 {
() => {
// Module: crate::utils::spanned
// Provides: {"impl_159"}
// Dependencies: {}
impl < 'a > From < Sp < & 'a str > > for Sp < String > { fn from (sp : Sp < & 'a str >) -> Self { Sp :: new (sp . val . into () , sp . span) } }
};
}
