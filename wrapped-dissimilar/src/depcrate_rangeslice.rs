// Generated macro for slice (function)
macro_rules! Depcrate_rangeslice {
() => {
// Module: crate::range
// Provides: {"slice"}
// Dependencies: {}
pub fn slice (range : Range) -> & [char] { & range . doc [range . offset .. range . offset + range . len] }
};
}
