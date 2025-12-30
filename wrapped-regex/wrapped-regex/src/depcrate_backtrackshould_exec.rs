// Generated macro for should_exec (function)
macro_rules! Depcrate_backtrackshould_exec {
() => {
// Module: crate::backtrack
// Provides: {"should_exec"}
// Dependencies: {}
# [doc = " Returns true iff the given regex and input should be executed by this"] # [doc = " engine with reasonable memory usage."] pub fn should_exec (num_insts : usize , text_len : usize) -> bool { let size = ((num_insts * (text_len + 1) + BIT_SIZE - 1) / BIT_SIZE) * 4 ; size <= MAX_SIZE_BYTES }
};
}
