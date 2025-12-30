// Generated macro for max_scalar_value (function)
macro_rules! Depcrate_utf8max_scalar_value {
() => {
// Module: crate::utf8
// Provides: {"max_scalar_value"}
// Dependencies: {}
fn max_scalar_value (nbytes : usize) -> u32 { match nbytes { 1 => 0x007F , 2 => 0x07FF , 3 => 0xFFFF , 4 => 0x0010_FFFF , _ => unreachable ! ("invalid UTF-8 byte sequence size") , } }
};
}
