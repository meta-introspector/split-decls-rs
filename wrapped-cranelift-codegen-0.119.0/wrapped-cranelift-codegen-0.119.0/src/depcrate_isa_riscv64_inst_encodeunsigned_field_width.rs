// Generated macro for unsigned_field_width (function)
macro_rules! Depcrate_isa_riscv64_inst_encodeunsigned_field_width {
() => {
// Module: crate::isa::riscv64::inst::encode
// Provides: {"unsigned_field_width"}
// Dependencies: {}
fn unsigned_field_width (value : u32 , width : u8) -> u32 { debug_assert_eq ! (value & (! 0 << width) , 0) ; value }
};
}
