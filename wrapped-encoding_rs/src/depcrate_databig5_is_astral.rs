// Generated macro for big5_is_astral (function)
macro_rules! Depcrate_databig5_is_astral {
() => {
// Module: crate::data
// Provides: {"big5_is_astral"}
// Dependencies: {}
# [inline (always)] pub fn big5_is_astral (rebased_pointer : usize) -> bool { (BIG5_ASTRALNESS [rebased_pointer >> 5] & (1 << (rebased_pointer & 0x1F))) != 0 }
};
}
