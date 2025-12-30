// Generated macro for impl_1322 (impl)
macro_rules! Depcrate_isa_x64_inst_argsimpl_1322 {
() => {
// Module: crate::isa::x64::inst::args
// Provides: {"impl_1322"}
// Dependencies: {}
impl AluRmROpcode { pub (crate) fn available_from (& self) -> SmallVec < [InstructionSet ; 2] > { match self { AluRmROpcode :: Andn => smallvec ! [InstructionSet :: BMI1] , AluRmROpcode :: Sarx | AluRmROpcode :: Shrx | AluRmROpcode :: Shlx | AluRmROpcode :: Bzhi => { smallvec ! [InstructionSet :: BMI2] } } } }
};
}
