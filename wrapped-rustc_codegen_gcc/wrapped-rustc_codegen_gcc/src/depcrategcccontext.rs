// Generated macro for GccContext (struct)
macro_rules! DepcrateGccContext {
() => {
// Module: crate
// Provides: {"GccContext"}
// Dependencies: {}
pub struct GccContext { context : Arc < SyncContext > , # [doc = " This field is needed in order to be able to set the flag -fPIC when necessary when doing"] # [doc = " LTO."] relocation_model : RelocModel , should_combine_object_files : bool , temp_dir : Option < TempDir > , }
};
}
