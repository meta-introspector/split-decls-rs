// Generated macro for private (module)
macro_rules! Depcrate_vecprivate {
() => {
// Module: crate::vec
// Provides: {"private"}
// Dependencies: {}
mod private { use crate :: { bf16 , f16 } ; # [cfg (feature = "alloc")] # [allow (unused_imports)] use alloc :: vec :: Vec ; pub trait SealedHalfFloatVec { } impl SealedHalfFloatVec for Vec < f16 > { } impl SealedHalfFloatVec for Vec < bf16 > { } pub trait SealedHalfBitsVec { } impl SealedHalfBitsVec for Vec < u16 > { } }
};
}
