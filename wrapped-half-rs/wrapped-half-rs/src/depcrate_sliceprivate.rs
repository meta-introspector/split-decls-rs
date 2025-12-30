// Generated macro for private (module)
macro_rules! Depcrate_sliceprivate {
() => {
// Module: crate::slice
// Provides: {"private"}
// Dependencies: {}
mod private { use crate :: { bf16 , f16 } ; pub trait SealedHalfFloatSlice { } impl SealedHalfFloatSlice for [f16] { } impl SealedHalfFloatSlice for [bf16] { } pub trait SealedHalfBitsSlice { } impl SealedHalfBitsSlice for [u16] { } }
};
}
