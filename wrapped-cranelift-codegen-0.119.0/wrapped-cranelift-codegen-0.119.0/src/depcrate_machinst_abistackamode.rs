// Generated macro for StackAMode (enum)
macro_rules! Depcrate_machinst_abiStackAMode {
() => {
// Module: crate::machinst::abi
// Provides: {"StackAMode"}
// Dependencies: {}
# [doc = " Abstract location for a machine-specific ABI impl to translate into the"] # [doc = " appropriate addressing mode."] # [derive (Clone , Copy , Debug)] pub enum StackAMode { # [doc = " Offset into the current frame's argument area."] IncomingArg (i64 , u32) , # [doc = " Offset within the stack slots in the current frame."] Slot (i64) , # [doc = " Offset into the callee frame's argument area."] OutgoingArg (i64) , }
};
}
