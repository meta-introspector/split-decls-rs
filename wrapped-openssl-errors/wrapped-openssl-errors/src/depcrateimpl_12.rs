// Generated macro for impl_12 (impl)
macro_rules! Depcrateimpl_12 {
() => {
// Module: crate
// Provides: {"impl_12"}
// Dependencies: {}
impl < T > Function < T > { # [doc = " This is not considered a part of the crate's public API, and is subject to change at any time."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The inner value must be valid for the lifetime of the process."] # [doc (hidden)] # [inline] pub const unsafe fn __from_raw (raw : FunctionInner) -> Function < T > { Function (raw , PhantomData) } # [doc = " This is not considered a part of the crate's public API, and is subject to change at any time."] # [doc (hidden)] # [inline] pub const fn __as_raw (& self) -> FunctionInner { self . 0 } }
};
}
