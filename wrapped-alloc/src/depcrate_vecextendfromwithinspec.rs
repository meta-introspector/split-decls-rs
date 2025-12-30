// Generated macro for ExtendFromWithinSpec (trait)
macro_rules! Depcrate_vecExtendFromWithinSpec {
() => {
// Module: crate::vec
// Provides: {"ExtendFromWithinSpec"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] trait ExtendFromWithinSpec { # [doc = " # Safety"] # [doc = ""] # [doc = " - `src` needs to be valid index"] # [doc = " - `self.capacity() - self.len()` must be `>= src.len()`"] unsafe fn spec_extend_from_within (& mut self , src : Range < usize >) ; }
};
}
