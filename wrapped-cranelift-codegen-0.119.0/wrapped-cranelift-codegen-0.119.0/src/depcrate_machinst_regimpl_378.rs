// Generated macro for impl_378 (impl)
macro_rules! Depcrate_machinst_regimpl_378 {
() => {
// Module: crate::machinst::reg
// Provides: {"impl_378"}
// Dependencies: {}
impl < 'a , F : Fn (VReg) -> VReg > OperandCollector < 'a , F > { # [doc = " Start gathering operands into one flattened operand array."] pub fn new (operands : & 'a mut Vec < Operand > , allocatable : PRegSet , renamer : F) -> Self { Self { operands , clobbers : PRegSet :: default () , allocatable , renamer , } } # [doc = " Finish the operand collection and return the tuple giving the"] # [doc = " range of indices in the flattened operand array, and the"] # [doc = " clobber set."] pub fn finish (self) -> (usize , PRegSet) { let end = self . operands . len () ; (end , self . clobbers) } }
};
}
