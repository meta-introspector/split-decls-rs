// Generated macro for impl_174 (impl)
macro_rules! Depcrate_helpersimpl_174 {
() => {
// Module: crate::helpers
// Provides: {"impl_174"}
// Dependencies: {}
impl I32CastError { # [doc = " Recovers the value saturated to `i32:::MIN..=i32::MAX`."] pub const fn saturate (self) -> i32 { match self { I32CastError :: BelowMin => i32 :: MIN , I32CastError :: AboveMax => i32 :: MAX , } } }
};
}
