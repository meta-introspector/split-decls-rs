// Generated macro for impl_129 (impl)
macro_rules! Depcrateimpl_129 {
() => {
// Module: crate
// Provides: {"impl_129"}
// Dependencies: {}
impl < C : ClientState > Clone for WalkDirOptions < C > { fn clone (& self) -> WalkDirOptions < C > { WalkDirOptions { sort : false , min_depth : self . min_depth , max_depth : self . max_depth , skip_hidden : self . skip_hidden , follow_links : self . follow_links , parallelism : self . parallelism . clone () , root_read_dir_state : self . root_read_dir_state . clone () , process_read_dir : self . process_read_dir . clone () , } } }
};
}
