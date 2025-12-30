// Generated macro for impl_2152 (impl)
macro_rules! Depcrate_isa_riscv64_inst_vectorimpl_2152 {
() => {
// Module: crate::isa::riscv64::inst::vector
// Provides: {"impl_2152"}
// Dependencies: {}
impl VecAluOpRImm5 { pub fn opcode (& self) -> u32 { 0x57 } pub fn funct3 (& self) -> u32 { self . category () . encode () } pub fn funct6 (& self) -> u32 { match self { VecAluOpRImm5 :: VmvVI => 0b010111 , } } pub fn category (& self) -> VecOpCategory { match self { VecAluOpRImm5 :: VmvVI => VecOpCategory :: OPIVI , } } # [doc = " Returns the auxiliary encoding field for the instruction, if any."] pub fn aux_encoding (& self) -> u32 { match self { VecAluOpRImm5 :: VmvVI => 0 , } } }
};
}
