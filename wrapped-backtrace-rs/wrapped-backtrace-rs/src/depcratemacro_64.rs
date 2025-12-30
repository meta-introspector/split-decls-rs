// Generated macro for macro_64 (macro)
macro_rules! Depcratemacro_64 {
() => {
// Module: crate
// Provides: {"macro_64"}
// Dependencies: {}
cfg_if :: cfg_if ! { if # [cfg (all (target_env = "sgx" , target_vendor = "fortanix" , not (feature = "std")))] { pub use self :: backtrace :: set_image_base ; } }
};
}
