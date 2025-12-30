// Generated macro for impl_2120 (impl)
macro_rules! Depcrate_isa_riscv64_inst_vectorimpl_2120 {
() => {
// Module: crate::isa::riscv64::inst::vector
// Provides: {"impl_2120"}
// Dependencies: {}
impl VecElementWidth { pub fn from_type (ty : Type) -> Self { Self :: from_bits (ty . lane_bits ()) } pub fn from_bits (bits : u32) -> Self { match bits { 8 => VecElementWidth :: E8 , 16 => VecElementWidth :: E16 , 32 => VecElementWidth :: E32 , 64 => VecElementWidth :: E64 , _ => panic ! ("Invalid number of bits for VecElementWidth: {bits}") , } } pub fn bits (& self) -> u32 { match self { VecElementWidth :: E8 => 8 , VecElementWidth :: E16 => 16 , VecElementWidth :: E32 => 32 , VecElementWidth :: E64 => 64 , } } pub fn encode (& self) -> u32 { match self { VecElementWidth :: E8 => 0b000 , VecElementWidth :: E16 => 0b001 , VecElementWidth :: E32 => 0b010 , VecElementWidth :: E64 => 0b011 , } } }
};
}
