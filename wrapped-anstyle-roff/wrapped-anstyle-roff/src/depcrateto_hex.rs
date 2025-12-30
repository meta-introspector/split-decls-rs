// Generated macro for to_hex (function)
macro_rules! Depcrateto_hex {
() => {
// Module: crate
// Provides: {"to_hex"}
// Dependencies: {}
fn to_hex (rgb : & RgbColor) -> String { let val : usize = ((rgb . 0 as usize) << 16) + ((rgb . 1 as usize) << 8) + (rgb . 2 as usize) ; format ! ("#{val:06x}") }
};
}
