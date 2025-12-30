// Generated macro for impl_193 (impl)
macro_rules! Depcrate_bb_word_fallbackimpl_193 {
() => {
// Module: crate::bb::word::fallback
// Provides: {"impl_193"}
// Dependencies: {}
impl WordOps for Word { # [inline] fn is_zero (self) -> BoolMask { use crate :: limb :: { Limb , LimbMask } ; prefixed_extern ! { fn LIMB_is_zero (limb : Limb) -> LimbMask ; } unsafe { LIMB_is_zero (self) } } }
};
}
