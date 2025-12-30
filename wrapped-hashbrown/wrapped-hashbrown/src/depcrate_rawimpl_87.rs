// Generated macro for impl_87 (impl)
macro_rules! Depcrate_rawimpl_87 {
() => {
// Module: crate::raw
// Provides: {"impl_87"}
// Dependencies: {}
# [cfg (feature = "nightly")] impl < T : Copy , A : Allocator + Clone > RawTableClone for RawTable < T , A > { # [cfg_attr (feature = "inline-more" , inline)] unsafe fn clone_from_spec (& mut self , source : & Self) { source . table . ctrl (0) . copy_to_nonoverlapping (self . table . ctrl (0) , self . table . num_ctrl_bytes ()) ; source . data_start () . as_ptr () . copy_to_nonoverlapping (self . data_start () . as_ptr () , self . table . buckets ()) ; self . table . items = source . table . items ; self . table . growth_left = source . table . growth_left ; } }
};
}
