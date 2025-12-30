// Generated macro for impl_26 (impl)
macro_rules! Depcrate_errorimpl_26 {
() => {
// Module: crate::error
// Provides: {"impl_26"}
// Dependencies: {}
impl < I : Copy > Error < & mut I > { # [doc = " Converts `Error<&mut I>` into `Error<I>` by copying."] pub fn copied (self) -> Error < I > { Error { input : * self . input , code : self . code , } } }
};
}
