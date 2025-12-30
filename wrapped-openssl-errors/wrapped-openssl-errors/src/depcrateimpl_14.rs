// Generated macro for impl_14 (impl)
macro_rules! Depcrateimpl_14 {
() => {
// Module: crate
// Provides: {"impl_14"}
// Dependencies: {}
impl < T > Reason < T > { # [doc = " This is not considered a part of the crate's public API, and is subject to change at any time."] # [doc (hidden)] # [inline] pub const fn __from_raw (raw : c_int) -> Reason < T > { Reason (raw , PhantomData) } # [doc = " This is not considered a part of the crate's public API, and is subject to change at any time."] # [doc (hidden)] # [inline] pub const fn __as_raw (& self) -> c_int { self . 0 } }
};
}
