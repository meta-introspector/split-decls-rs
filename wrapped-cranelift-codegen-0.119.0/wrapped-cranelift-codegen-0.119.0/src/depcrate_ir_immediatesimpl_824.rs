// Generated macro for impl_824 (impl)
macro_rules! Depcrate_ir_immediatesimpl_824 {
() => {
// Module: crate::ir::immediates
// Provides: {"impl_824"}
// Dependencies: {}
impl From < & [u8] > for V128Imm { fn from (slice : & [u8]) -> Self { assert_eq ! (slice . len () , 16) ; let mut buffer = [0 ; 16] ; buffer . copy_from_slice (slice) ; Self (buffer) } }
};
}
