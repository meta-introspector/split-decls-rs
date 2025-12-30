// Generated macro for impl_391 (impl)
macro_rules! Depcrate_machinstimpl_391 {
() => {
// Module: crate::machinst
// Provides: {"impl_391"}
// Dependencies: {}
impl CompiledCodeStencil { # [doc = " Apply function parameters to finalize a stencil into its final form."] pub fn apply_params (self , params : & FunctionParameters) -> CompiledCode { CompiledCode { buffer : self . buffer . apply_base_srcloc (params . base_srcloc ()) , frame_size : self . frame_size , vcode : self . vcode , value_labels_ranges : self . value_labels_ranges , sized_stackslot_offsets : self . sized_stackslot_offsets , dynamic_stackslot_offsets : self . dynamic_stackslot_offsets , bb_starts : self . bb_starts , bb_edges : self . bb_edges , } } }
};
}
