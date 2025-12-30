// Generated macro for impl_1325 (impl)
macro_rules! Depcrate_isa_x64_inst_argsimpl_1325 {
() => {
// Module: crate::isa::x64::inst::args
// Provides: {"impl_1325"}
// Dependencies: {}
impl UnaryRmROpcode { pub (crate) fn available_from (& self) -> SmallVec < [InstructionSet ; 2] > { match self { UnaryRmROpcode :: Bsr | UnaryRmROpcode :: Bsf => smallvec ! [] , UnaryRmROpcode :: Lzcnt => smallvec ! [InstructionSet :: Lzcnt] , UnaryRmROpcode :: Tzcnt => smallvec ! [InstructionSet :: BMI1] , UnaryRmROpcode :: Popcnt => smallvec ! [InstructionSet :: Popcnt] , } } }
};
}
