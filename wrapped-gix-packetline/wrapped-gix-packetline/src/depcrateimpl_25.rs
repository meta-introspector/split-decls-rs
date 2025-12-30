// Generated macro for impl_25 (impl)
macro_rules! Depcrateimpl_25 {
() => {
// Module: crate
// Provides: {"impl_25"}
// Dependencies: {}
impl < 'a > TextRef < 'a > { # [doc = " Return this instance's data."] pub fn as_slice (& self) -> & 'a [u8] { self . 0 } # [doc = " Return this instance's data as [`BStr`]."] pub fn as_bstr (& self) -> & 'a BStr { self . 0 . into () } }
};
}
