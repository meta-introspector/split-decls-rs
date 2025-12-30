// Generated macro for black_box (function)
macro_rules! Depcrateblack_box {
() => {
// Module: crate
// Provides: {"black_box"}
// Dependencies: {}
fn black_box < T > (dummy : T) -> T { unsafe { let ret = ptr :: read_volatile (& dummy) ; mem :: forget (dummy) ; ret } }
};
}
