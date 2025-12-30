// Generated macro for as_char (function)
macro_rules! Depcrate_utilas_char {
() => {
// Module: crate::util
// Provides: {"as_char"}
// Dependencies: {}
# [doc = " Unchecked conversion to `char`."] pub fn as_char (ch : u32) -> char { debug_assert ! (char :: from_u32 (ch) . is_some ()) ; unsafe { mem :: transmute (ch) } }
};
}
