// Generated macro for VCodeInst (trait)
macro_rules! Depcrate_machinst_vcodeVCodeInst {
() => {
// Module: crate::machinst::vcode
// Provides: {"VCodeInst"}
// Dependencies: {}
# [doc = " VCodeInst wraps all requirements for a MachInst to be in VCode: it must be"] # [doc = " a `MachInst` and it must be able to emit itself at least to a `SizeCodeSink`."] pub trait VCodeInst : MachInst + MachInstEmit { }
};
}
