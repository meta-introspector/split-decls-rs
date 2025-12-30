// Generated macro for create (function)
macro_rules! Depcrate_pack_multi_indexcreate {
() => {
// Module: crate::pack::multi_index
// Provides: {"create"}
// Dependencies: {}
pub fn create (index_paths : Vec < PathBuf > , output_path : PathBuf , mut progress : impl NestedProgress + 'static , should_interrupt : & AtomicBool , object_hash : gix :: hash :: Kind ,) -> anyhow :: Result < () > { let mut out = BufWriter :: new (gix :: lock :: File :: acquire_to_update_resource (output_path , gix :: lock :: acquire :: Fail :: Immediately , None ,) ?) ; gix :: odb :: pack :: multi_index :: File :: write_from_index_paths (index_paths , & mut out , & mut progress , should_interrupt , gix :: odb :: pack :: multi_index :: write :: Options { object_hash } ,) ? ; out . into_inner () ? . commit () ? ; Ok (()) }
};
}
