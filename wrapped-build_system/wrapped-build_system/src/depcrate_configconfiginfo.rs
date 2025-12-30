// Generated macro for ConfigInfo (struct)
macro_rules! Depcrate_configConfigInfo {
() => {
// Module: crate::config
// Provides: {"ConfigInfo"}
// Dependencies: {}
# [derive (Default , Debug , Clone)] pub struct ConfigInfo { pub target : String , pub target_triple : String , pub host_triple : String , pub rustc_command : Vec < String > , pub run_in_vm : bool , pub cargo_target_dir : String , pub dylib_ext : String , pub sysroot_release_channel : bool , pub channel : Channel , pub sysroot_panic_abort : bool , pub cg_backend_path : String , pub sysroot_path : String , pub gcc_path : Option < String > , config_file : Option < String > , cg_gcc_path : Option < PathBuf > , pub no_download : bool , pub no_default_features : bool , pub backend : Option < String > , pub features : Vec < String > , }
};
}
