// Generated macro for OutputSize (trait)
macro_rules! Depcrate_variantsOutputSize {
() => {
// Module: crate::variants
// Provides: {"OutputSize"}
// Dependencies: {}
# [doc = " Trait implemented for output sizes supported by `bash-hash`."] # [doc = ""] # [doc = " Supported output sizes form the following list: U4, U8, ..., U60, U64."] pub trait OutputSize : ArraySize + Sealed { # [doc = " Block size in bytes computed as `192 - 2 * OutputSize`."] type BlockSize : BlockSizes ; }
};
}
