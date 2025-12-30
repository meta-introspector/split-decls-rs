// Generated macro for usize (module)
macro_rules! Depcrate_wordusize {
() => {
// Module: crate::word
// Provides: {"usize"}
// Dependencies: {}
pub mod usize { # [cfg (target_pointer_width = "16")] super :: simple_word_impl ! (usize , u32) ; # [cfg (target_pointer_width = "32")] super :: simple_word_impl ! (usize , u64) ; # [cfg (target_pointer_width = "64")] super :: simple_word_impl ! (usize , u128) ; }
};
}
