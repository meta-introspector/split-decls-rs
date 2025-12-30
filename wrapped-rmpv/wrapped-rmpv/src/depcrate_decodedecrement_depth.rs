// Generated macro for decrement_depth (function)
macro_rules! Depcrate_decodedecrement_depth {
() => {
// Module: crate::decode
// Provides: {"decrement_depth"}
// Dependencies: {}
# [inline] fn decrement_depth (depth : u16) -> Result < u16 , Error > { depth . checked_sub (1) . ok_or (Error :: DepthLimitExceeded) }
};
}
