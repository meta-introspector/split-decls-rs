// Generated macro for MachBufferConstant (struct)
macro_rules! Depcrate_machinst_bufferMachBufferConstant {
() => {
// Module: crate::machinst::buffer
// Provides: {"MachBufferConstant"}
// Dependencies: {}
# [doc = " Metadata about a constant."] struct MachBufferConstant { # [doc = " A label which has not yet been bound which can be used for this"] # [doc = " constant."] # [doc = ""] # [doc = " This is lazily created when a label is requested for a constant and is"] # [doc = " cleared when a constant is emitted."] upcoming_label : Option < MachLabel > , # [doc = " Required alignment."] align : CodeOffset , # [doc = " The byte size of this constant."] size : usize , }
};
}
