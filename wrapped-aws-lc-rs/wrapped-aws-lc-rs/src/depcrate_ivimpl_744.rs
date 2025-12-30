// Generated macro for impl_744 (impl)
macro_rules! Depcrate_ivimpl_744 {
() => {
// Module: crate::iv
// Provides: {"impl_744"}
// Dependencies: {}
impl < const L : usize > From < & [u8 ; L] > for FixedLength < L > { # [inline] fn from (bytes : & [u8 ; L]) -> Self { FixedLength (bytes . to_owned ()) } }
};
}
