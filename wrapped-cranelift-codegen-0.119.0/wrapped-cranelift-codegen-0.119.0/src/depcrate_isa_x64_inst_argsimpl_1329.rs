// Generated macro for impl_1329 (impl)
macro_rules! Depcrate_isa_x64_inst_argsimpl_1329 {
() => {
// Module: crate::isa::x64::inst::args
// Provides: {"impl_1329"}
// Dependencies: {}
impl UnaryRmRVexOpcode { pub (crate) fn available_from (& self) -> SmallVec < [InstructionSet ; 2] > { match self { UnaryRmRVexOpcode :: Blsi | UnaryRmRVexOpcode :: Blsmsk | UnaryRmRVexOpcode :: Blsr => { smallvec ! [InstructionSet :: BMI1] } } } }
};
}
