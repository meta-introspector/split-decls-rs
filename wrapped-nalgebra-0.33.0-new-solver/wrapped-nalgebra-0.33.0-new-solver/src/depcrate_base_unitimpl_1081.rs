// Generated macro for impl_1081 (impl)
macro_rules! Depcrate_base_unitimpl_1081 {
() => {
// Module: crate::base::unit
// Provides: {"impl_1081"}
// Dependencies: {}
# [doc = " # Data extraction and construction without normalization"] impl < T > Unit < T > { # [doc = " Wraps the given value, assuming it is already normalized."] # [inline] pub const fn new_unchecked (value : T) -> Self { Unit { value } } # [doc = " Wraps the given reference, assuming it is already normalized."] # [inline] pub fn from_ref_unchecked (value : & T) -> & Self { unsafe { & * (value as * const T as * const Self) } } # [doc = " Retrieves the underlying value."] # [inline] pub fn into_inner (self) -> T { self . value } # [doc = " Retrieves the underlying value."] # [doc = " Deprecated: use [`Unit::into_inner`] instead."] # [deprecated (note = "use `.into_inner()` instead")] # [inline] pub fn unwrap (self) -> T { self . value } # [doc = " Returns a mutable reference to the underlying value. This is `_unchecked` because modifying"] # [doc = " the underlying value in such a way that it no longer has unit length may lead to unexpected"] # [doc = " results."] # [inline] pub fn as_mut_unchecked (& mut self) -> & mut T { & mut self . value } }
};
}
