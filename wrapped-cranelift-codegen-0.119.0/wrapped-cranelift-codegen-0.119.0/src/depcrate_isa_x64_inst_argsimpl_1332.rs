// Generated macro for impl_1332 (impl)
macro_rules! Depcrate_isa_x64_inst_argsimpl_1332 {
() => {
// Module: crate::isa::x64::inst::args
// Provides: {"impl_1332"}
// Dependencies: {}
impl UnaryRmRImmVexOpcode { pub (crate) fn available_from (& self) -> SmallVec < [InstructionSet ; 2] > { match self { UnaryRmRImmVexOpcode :: Rorx => { smallvec ! [InstructionSet :: BMI2] } } } }
};
}
