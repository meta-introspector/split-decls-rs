// Generated macro for impl_130 (impl)
macro_rules! Depcrate_fullimpl_130 {
() => {
// Module: crate::full
// Provides: {"impl_130"}
// Dependencies: {}
impl < D > Full < D > { # [doc = " Consumes this `Full`, returning the wrapped value."] # [doc = ""] # [doc = " If this body has already been polled, this returns `None`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use http_body_util::Full;"] # [doc = " const DATA: &[u8] = b\"hello\";"] # [doc = " let full = Full::new(DATA);"] # [doc = " assert_eq!(full.into_inner(), Some(\"hello\").map(str::as_bytes));"] # [doc = " ```"] pub fn into_inner (self) -> Option < D > { self . data } }
};
}
