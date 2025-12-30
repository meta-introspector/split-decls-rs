// Generated macro for EmitResult (struct)
macro_rules! Depcrate_machinst_vcodeEmitResult {
() => {
// Module: crate::machinst::vcode
// Provides: {"EmitResult"}
// Dependencies: {}
# [doc = " The result of `VCode::emit`. Contains all information computed"] # [doc = " during emission: actual machine code, optionally a disassembly,"] # [doc = " and optionally metadata about the code layout."] pub struct EmitResult { # [doc = " The MachBuffer containing the machine code."] pub buffer : MachBufferFinalized < Stencil > , # [doc = " Offset of each basic block, recorded during emission. Computed"] # [doc = " only if `debug_value_labels` is non-empty."] pub bb_offsets : Vec < CodeOffset > , # [doc = " Final basic-block edges, in terms of code offsets of"] # [doc = " bb-starts. Computed only if `debug_value_labels` is non-empty."] pub bb_edges : Vec < (CodeOffset , CodeOffset) > , # [doc = " Final length of function body."] pub func_body_len : CodeOffset , # [doc = " The pretty-printed disassembly, if any. This uses the same"] # [doc = " pretty-printing for MachInsts as the pre-regalloc VCode Debug"] # [doc = " implementation, but additionally includes the prologue and"] # [doc = " epilogue(s), and makes use of the regalloc results."] pub disasm : Option < String > , # [doc = " Offsets of sized stackslots."] pub sized_stackslot_offsets : PrimaryMap < StackSlot , u32 > , # [doc = " Offsets of dynamic stackslots."] pub dynamic_stackslot_offsets : PrimaryMap < DynamicStackSlot , u32 > , # [doc = " Value-labels information (debug metadata)."] pub value_labels_ranges : ValueLabelsRanges , # [doc = " Stack frame size."] pub frame_size : u32 , }
};
}
