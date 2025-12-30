// Generated macro for impl_93 (impl)
macro_rules! Depcrate_cache_paddedimpl_93 {
() => {
// Module: crate::cache_padded
// Provides: {"impl_93"}
// Dependencies: {}
impl < T > CachePadded < T > { # [doc = " Pads and aligns a value to the length of a cache line."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_utils::CachePadded;"] # [doc = ""] # [doc = " let padded_value = CachePadded::new(1);"] # [doc = " ```"] pub const fn new (t : T) -> Self { Self { value : t } } # [doc = " Returns the inner value."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_utils::CachePadded;"] # [doc = ""] # [doc = " let padded_value = CachePadded::new(7);"] # [doc = " let value = padded_value.into_inner();"] # [doc = " assert_eq!(value, 7);"] # [doc = " ```"] pub fn into_inner (self) -> T { self . value } }
};
}
