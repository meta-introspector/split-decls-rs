// Generated macro for impl_10 (impl)
macro_rules! Depcrate_instructionimpl_10 {
() => {
// Module: crate::instruction
// Provides: {"impl_10"}
// Dependencies: {}
impl Instruction < '_ > { # [doc = " Derive the mode of operation from this instruction."] pub fn operation (& self) -> Operation { match self { Instruction :: Push (_) => Operation :: Push , Instruction :: Fetch (_) => Operation :: Fetch , } } }
};
}
