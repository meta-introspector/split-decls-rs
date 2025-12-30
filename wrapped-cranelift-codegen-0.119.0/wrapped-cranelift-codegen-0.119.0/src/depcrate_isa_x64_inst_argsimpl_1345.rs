// Generated macro for impl_1345 (impl)
macro_rules! Depcrate_isa_x64_inst_argsimpl_1345 {
() => {
// Module: crate::isa::x64::inst::args
// Provides: {"impl_1345"}
// Dependencies: {}
impl Avx512Opcode { # [doc = " Which `InstructionSet`s support the opcode?"] pub (crate) fn available_from (& self) -> SmallVec < [InstructionSet ; 2] > { match self { Avx512Opcode :: Vcvtudq2ps | Avx512Opcode :: Vpabsq | Avx512Opcode :: Vpsraq | Avx512Opcode :: VpsraqImm => { smallvec ! [InstructionSet :: AVX512F , InstructionSet :: AVX512VL] } Avx512Opcode :: Vpermi2b => { smallvec ! [InstructionSet :: AVX512VL , InstructionSet :: AVX512VBMI] } Avx512Opcode :: Vpmullq => smallvec ! [InstructionSet :: AVX512VL , InstructionSet :: AVX512DQ] , Avx512Opcode :: Vpopcntb => { smallvec ! [InstructionSet :: AVX512VL , InstructionSet :: AVX512BITALG] } } } # [doc = " What is the \"TupleType\" of this opcode, which affects the scaling factor"] # [doc = " for 8-bit displacements when this instruction uses memory operands."] # [doc = ""] # [doc = " This can be found in the encoding table for each instruction and is"] # [doc = " interpreted according to Table 2-34 and 2-35 in the Intel instruction"] # [doc = " manual."] pub fn tuple_type (& self) -> Avx512TupleType { use Avx512Opcode :: * ; use Avx512TupleType :: * ; match self { Vcvtudq2ps | Vpabsq | Vpmullq | VpsraqImm => Full , Vpermi2b | Vpopcntb => FullMem , Vpsraq => Mem128 , } } }
};
}
