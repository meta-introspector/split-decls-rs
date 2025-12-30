// Generated macro for impl_14 (impl)
macro_rules! Depcrate_csprojimpl_14 {
() => {
// Module: crate::csproj
// Provides: {"impl_14"}
// Dependencies: {}
impl CSProject { pub fn new (dir : PathBuf , name : & str , world_name : & str) -> CSProjectLLVMBuilder { CSProjectLLVMBuilder { name : name . to_string () , dir , aot : false , clean_targets : false , world_name : world_name . to_string () , binary : false , } } pub fn new_mono (dir : PathBuf , name : & str , world_name : & str) -> CSProjectMonoBuilder { CSProjectMonoBuilder { name : name . to_string () , dir , aot : false , clean_targets : false , world_name : world_name . to_string () , } } }
};
}
