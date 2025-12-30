// Generated macro for CompiledCodeBase (struct)
macro_rules! Depcrate_machinstCompiledCodeBase {
() => {
// Module: crate::machinst
// Provides: {"CompiledCodeBase"}
// Dependencies: {}
# [doc = " The result of a `MachBackend::compile_function()` call. Contains machine"] # [doc = " code (as bytes) and a disassembly, if requested."] # [derive (PartialEq , Debug , Clone)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub struct CompiledCodeBase < T : CompilePhase > { # [doc = " Machine code."] pub buffer : MachBufferFinalized < T > , # [doc = " Size of stack frame, in bytes."] pub frame_size : u32 , # [doc = " Disassembly, if requested."] pub vcode : Option < String > , # [doc = " Debug info: value labels to registers/stackslots at code offsets."] pub value_labels_ranges : ValueLabelsRanges , # [doc = " Debug info: stackslots to stack pointer offsets."] pub sized_stackslot_offsets : PrimaryMap < StackSlot , u32 > , # [doc = " Debug info: stackslots to stack pointer offsets."] pub dynamic_stackslot_offsets : PrimaryMap < DynamicStackSlot , u32 > , # [doc = " Basic-block layout info: block start offsets."] # [doc = ""] # [doc = " This info is generated only if the `machine_code_cfg_info`"] # [doc = " flag is set."] pub bb_starts : Vec < CodeOffset > , # [doc = " Basic-block layout info: block edges. Each edge is `(from,"] # [doc = " to)`, where `from` and `to` are basic-block start offsets of"] # [doc = " the respective blocks."] # [doc = ""] # [doc = " This info is generated only if the `machine_code_cfg_info`"] # [doc = " flag is set."] pub bb_edges : Vec < (CodeOffset , CodeOffset) > , }
};
}
