// Generated macro for shift_for_type (function)
macro_rules! Depcrate_isa_aarch64_inst_argsshift_for_type {
() => {
// Module: crate::isa::aarch64::inst::args
// Provides: {"shift_for_type"}
// Dependencies: {}
fn shift_for_type (size_bytes : u8) -> usize { match size_bytes { 1 => 0 , 2 => 1 , 4 => 2 , 8 => 3 , 16 => 4 , _ => panic ! ("unknown type size: {size_bytes}") , } }
};
}
