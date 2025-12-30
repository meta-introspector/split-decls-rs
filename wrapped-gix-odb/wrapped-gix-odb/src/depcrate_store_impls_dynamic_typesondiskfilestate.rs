// Generated macro for OnDiskFileState (enum)
macro_rules! Depcrate_store_impls_dynamic_typesOnDiskFileState {
() => {
// Module: crate::store_impls::dynamic::types
// Provides: {"OnDiskFileState"}
// Dependencies: {}
# [derive (Clone)] pub (crate) enum OnDiskFileState < T : Clone > { # [doc = " The file is on disk and can be loaded from there."] Unloaded , Loaded (T) , # [doc = " The file was loaded, but appeared to be missing on disk after reconciling our state with what's on disk."] # [doc = " As there were handles that required pack-id stability we had to keep the item to allow finding it on later"] # [doc = " lookups."] Garbage (T) , # [doc = " File is missing on disk and could not be loaded when we tried or turned missing after reconciling our state."] Missing , }
};
}
