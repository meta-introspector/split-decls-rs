// Generated macro for prelude (module)
macro_rules! Depcrateprelude {
() => {
// Module: crate
// Provides: {"prelude"}
// Dependencies: {}
# [doc = " A collection of the most used items and traits in this crate for easy importing."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " use half::prelude::*;"] # [doc = " ```"] pub mod prelude { # [doc (no_inline)] pub use crate :: { bf16 , f16 } ; # [cfg (not (target_arch = "spirv"))] # [doc (no_inline)] pub use crate :: slice :: { HalfBitsSliceExt , HalfFloatSliceExt } ; # [cfg (feature = "alloc")] # [doc (no_inline)] pub use crate :: vec :: { HalfBitsVecExt , HalfFloatVecExt } ; }
};
}
