// Generated macro for impl_2137 (impl)
macro_rules! Depcrate_isa_riscv64_inst_vectorimpl_2137 {
() => {
// Module: crate::isa::riscv64::inst::vector
// Provides: {"impl_2137"}
// Dependencies: {}
impl VecAluOpRRRR { pub fn opcode (& self) -> u32 { 0x57 } pub fn funct3 (& self) -> u32 { self . category () . encode () } pub fn funct6 (& self) -> u32 { match self { VecAluOpRRRR :: VmaccVV | VecAluOpRRRR :: VmaccVX => 0b101101 , VecAluOpRRRR :: VnmsacVV | VecAluOpRRRR :: VnmsacVX => 0b101111 , VecAluOpRRRR :: VfmaccVV | VecAluOpRRRR :: VfmaccVF => 0b101100 , VecAluOpRRRR :: VfnmaccVV | VecAluOpRRRR :: VfnmaccVF => 0b101101 , VecAluOpRRRR :: VfmsacVV | VecAluOpRRRR :: VfmsacVF => 0b101110 , VecAluOpRRRR :: VfnmsacVV | VecAluOpRRRR :: VfnmsacVF => 0b101111 , VecAluOpRRRR :: Vslide1upVX => 0b001110 , } } pub fn category (& self) -> VecOpCategory { match self { VecAluOpRRRR :: VmaccVV | VecAluOpRRRR :: VnmsacVV => VecOpCategory :: OPMVV , VecAluOpRRRR :: VmaccVX | VecAluOpRRRR :: VnmsacVX | VecAluOpRRRR :: Vslide1upVX => { VecOpCategory :: OPMVX } VecAluOpRRRR :: VfmaccVV | VecAluOpRRRR :: VfnmaccVV | VecAluOpRRRR :: VfmsacVV | VecAluOpRRRR :: VfnmsacVV => VecOpCategory :: OPFVV , VecAluOpRRRR :: VfmaccVF | VecAluOpRRRR :: VfnmaccVF | VecAluOpRRRR :: VfmsacVF | VecAluOpRRRR :: VfnmsacVF => VecOpCategory :: OPFVF , } } pub fn vs1_regclass (& self) -> RegClass { match self . category () { VecOpCategory :: OPMVV | VecOpCategory :: OPFVV => RegClass :: Vector , VecOpCategory :: OPMVX => RegClass :: Int , VecOpCategory :: OPFVF => RegClass :: Float , _ => unreachable ! () , } } }
};
}
