// Generated macro for black_box (function)
macro_rules! Depcrate_black_boxblack_box {
() => {
// Module: crate::black_box
// Provides: {"black_box"}
// Dependencies: {}
pub fn black_box < T > (dummy : T) -> T { unsafe { let ret = std :: ptr :: read_volatile (& dummy) ; std :: mem :: forget (dummy) ; ret } }
};
}
