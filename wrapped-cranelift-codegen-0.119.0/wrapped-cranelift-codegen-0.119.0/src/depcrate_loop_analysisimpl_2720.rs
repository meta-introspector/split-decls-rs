// Generated macro for impl_2720 (impl)
macro_rules! Depcrate_loop_analysisimpl_2720 {
() => {
// Module: crate::loop_analysis
// Provides: {"impl_2720"}
// Dependencies: {}
impl LoopLevel { const INVALID : u8 = u8 :: MAX ; # [doc = " Get the root level (no loop)."] pub fn root () -> Self { Self (0) } # [doc = " Get the loop level."] pub fn level (self) -> usize { self . 0 as usize } # [doc = " Invalid loop level."] pub fn invalid () -> Self { Self (Self :: INVALID) } # [doc = " One loop level deeper."] pub fn inc (self) -> Self { if self . 0 == (Self :: INVALID - 1) { self } else { Self (self . 0 + 1) } } # [doc = " A clamped loop level from a larger-width (usize) depth."] pub fn clamped (level : usize) -> Self { Self (u8 :: try_from (std :: cmp :: min (level , (Self :: INVALID as usize) - 1)) . expect ("Clamped value must always convert") ,) } }
};
}
