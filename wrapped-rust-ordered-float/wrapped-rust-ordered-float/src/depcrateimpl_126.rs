// Generated macro for impl_126 (impl)
macro_rules! Depcrateimpl_126 {
() => {
// Module: crate
// Provides: {"impl_126"}
// Dependencies: {}
impl < T > NotNan < T > { # [doc = " Get the value out."] # [inline] pub fn into_inner (self) -> T { self . 0 } # [doc = " Create a `NotNan` value from a value that is guaranteed to not be NaN"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Behaviour is undefined if `val` is NaN"] # [inline] pub const unsafe fn new_unchecked (val : T) -> Self { NotNan (val) } # [doc = " Create a `NotNan` value from a value that is guaranteed to not be NaN"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Behaviour is undefined if `val` is NaN"] # [deprecated (since = "2.5.0" , note = "Please use the new_unchecked function instead.")] # [inline] pub const unsafe fn unchecked_new (val : T) -> Self { Self :: new_unchecked (val) } }
};
}
