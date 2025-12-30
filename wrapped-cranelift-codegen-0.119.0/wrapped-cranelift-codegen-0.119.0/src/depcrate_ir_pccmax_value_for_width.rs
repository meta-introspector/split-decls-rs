// Generated macro for max_value_for_width (function)
macro_rules! Depcrate_ir_pccmax_value_for_width {
() => {
// Module: crate::ir::pcc
// Provides: {"max_value_for_width"}
// Dependencies: {}
fn max_value_for_width (bits : u16) -> u64 { assert ! (bits <= 64) ; if bits == 64 { u64 :: MAX } else { (1u64 << bits) - 1 } }
};
}
