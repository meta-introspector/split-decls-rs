// Generated macro for end_of_input (function)
macro_rules! Depcrate_inflate_coreend_of_input {
() => {
// Module: crate::inflate::core
// Provides: {"end_of_input"}
// Dependencies: {}
# [inline] const fn end_of_input (flags : u32) -> Action { Action :: End (if flags & TINFL_FLAG_HAS_MORE_INPUT != 0 { TINFLStatus :: NeedsMoreInput } else { TINFLStatus :: FailedCannotMakeProgress }) }
};
}
