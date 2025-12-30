// Generated macro for OperandCollector (struct)
macro_rules! Depcrate_machinst_regOperandCollector {
() => {
// Module: crate::machinst::reg
// Provides: {"OperandCollector"}
// Dependencies: {}
# [doc = " An OperandCollector is a wrapper around a Vec of Operands"] # [doc = " (flattened array for a whole sequence of instructions) that"] # [doc = " gathers operands from a single instruction and provides the range"] # [doc = " in the flattened array."] # [derive (Debug)] pub struct OperandCollector < 'a , F : Fn (VReg) -> VReg > { operands : & 'a mut Vec < Operand > , clobbers : PRegSet , # [doc = " The subset of physical registers that are allocatable."] allocatable : PRegSet , renamer : F , }
};
}
