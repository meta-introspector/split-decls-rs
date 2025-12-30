// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { fuzz ! (| data : & [u8] | { let _ = decompress_all (data) ; }) ; }
};
}
