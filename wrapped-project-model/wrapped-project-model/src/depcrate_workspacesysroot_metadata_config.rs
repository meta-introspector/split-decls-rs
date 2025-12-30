// Generated macro for sysroot_metadata_config (function)
macro_rules! Depcrate_workspacesysroot_metadata_config {
() => {
// Module: crate::workspace
// Provides: {"sysroot_metadata_config"}
// Dependencies: {}
fn sysroot_metadata_config (config : & CargoConfig , targets : & [String] , toolchain_version : Option < Version > ,) -> CargoMetadataConfig { CargoMetadataConfig { features : Default :: default () , targets : targets . to_vec () , extra_args : Default :: default () , extra_env : config . extra_env . clone () , toolchain_version , kind : "sysroot" , } }
};
}
