// Generated macro for impl_3231 (impl)
macro_rules! Depcrate_large_stack_framesimpl_3231 {
() => {
// Module: crate::large_stack_frames
// Provides: {"impl_3231"}
// Dependencies: {}
impl Space { pub fn exceeds_limit (self , limit : u64) -> bool { match self { Self :: Used (used) => used > limit , Self :: Overflow => true , } } }
};
}
