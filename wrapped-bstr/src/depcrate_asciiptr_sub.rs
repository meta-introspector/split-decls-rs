// Generated macro for ptr_sub (function)
macro_rules! Depcrate_asciiptr_sub {
() => {
// Module: crate::ascii
// Provides: {"ptr_sub"}
// Dependencies: {}
# [doc = " Decrement the given pointer by the given amount."] unsafe fn ptr_sub (ptr : * const u8 , amt : usize) -> * const u8 { ptr . sub (amt) }
};
}
