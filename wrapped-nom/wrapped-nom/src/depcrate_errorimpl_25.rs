// Generated macro for impl_25 (impl)
macro_rules! Depcrate_errorimpl_25 {
() => {
// Module: crate::error
// Provides: {"impl_25"}
// Dependencies: {}
impl < I : Copy > Error < & I > { # [doc = " Converts `Error<&I>` into `Error<I>` by copying."] pub fn copied (self) -> Error < I > { Error { input : * self . input , code : self . code , } } }
};
}
