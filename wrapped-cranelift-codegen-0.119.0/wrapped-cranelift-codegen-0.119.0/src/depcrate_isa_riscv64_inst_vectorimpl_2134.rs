// Generated macro for impl_2134 (impl)
macro_rules! Depcrate_isa_riscv64_inst_vectorimpl_2134 {
() => {
// Module: crate::isa::riscv64::inst::vector
// Provides: {"impl_2134"}
// Dependencies: {}
impl VecOpCategory { pub fn encode (& self) -> u32 { match self { VecOpCategory :: OPIVV => 0b000 , VecOpCategory :: OPFVV => 0b001 , VecOpCategory :: OPMVV => 0b010 , VecOpCategory :: OPIVI => 0b011 , VecOpCategory :: OPIVX => 0b100 , VecOpCategory :: OPFVF => 0b101 , VecOpCategory :: OPMVX => 0b110 , VecOpCategory :: OPCFG => 0b111 , } } }
};
}
