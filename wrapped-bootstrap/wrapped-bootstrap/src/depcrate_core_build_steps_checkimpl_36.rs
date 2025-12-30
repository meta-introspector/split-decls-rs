// Generated macro for impl_36 (impl)
macro_rules! Depcrate_core_build_steps_checkimpl_36 {
() => {
// Module: crate::core::build_steps::check
// Provides: {"impl_36"}
// Dependencies: {}
impl RmetaSysroot { # [doc = " Copy rmeta artifacts from the given `stamp` into a sysroot located at `directory`."] fn from_stamp (builder : & Builder < '_ > , stamp : BuildStamp , target : TargetSelection , directory : & Path ,) -> Self { let host_dir = directory . join ("host") ; let target_dir = directory . join (target) ; let _ = fs :: remove_dir_all (directory) ; t ! (fs :: create_dir_all (directory)) ; add_to_sysroot (builder , & target_dir , & host_dir , & stamp) ; Self { host_dir , target_dir } } # [doc = " Configure the given cargo invocation so that the compiled crate will be able to use"] # [doc = " rustc .rmeta artifacts that were previously generated."] fn configure_cargo (& self , cargo : & mut Cargo) { cargo . append_to_env ("RUSTC_ADDITIONAL_SYSROOT_PATHS" , format ! ("{},{}" , self . host_dir . to_str () . unwrap () , self . target_dir . to_str () . unwrap ()) , "," ,) ; } }
};
}
