// Generated macro for ptr_add (function)
macro_rules! Depcrate_asciiptr_add {
() => {
// Module: crate::ascii
// Provides: {"ptr_add"}
// Dependencies: {}
# [doc = " Increment the given pointer by the given amount."] unsafe fn ptr_add (ptr : * const u8 , amt : usize) -> * const u8 { ptr . add (amt) }
};
}
