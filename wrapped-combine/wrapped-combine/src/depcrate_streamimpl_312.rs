// Generated macro for impl_312 (impl)
macro_rules! Depcrate_streamimpl_312 {
() => {
// Module: crate::stream
// Provides: {"impl_312"}
// Dependencies: {}
impl < 's , S > From < & 's mut S > for & 's mut CompleteStream < S > { fn from (t : & 's mut S) -> Self { unsafe { & mut * (t as * mut S as * mut CompleteStream < S >) } } }
};
}
