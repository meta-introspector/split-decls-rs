// Generated macro for impl_2140 (impl)
macro_rules! Depcrate_isa_riscv64_inst_vectorimpl_2140 {
() => {
// Module: crate::isa::riscv64::inst::vector
// Provides: {"impl_2140"}
// Dependencies: {}
impl VecAluOpRRRImm5 { pub fn opcode (& self) -> u32 { 0x57 } pub fn funct3 (& self) -> u32 { self . category () . encode () } pub fn funct6 (& self) -> u32 { match self { VecAluOpRRRImm5 :: VslideupVI => 0b001110 , } } pub fn category (& self) -> VecOpCategory { match self { VecAluOpRRRImm5 :: VslideupVI => VecOpCategory :: OPIVI , } } pub fn imm_is_unsigned (& self) -> bool { match self { VecAluOpRRRImm5 :: VslideupVI => true , } } }
};
}
