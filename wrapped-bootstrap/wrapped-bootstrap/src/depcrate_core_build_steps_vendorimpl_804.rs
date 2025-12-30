// Generated macro for impl_804 (impl)
macro_rules! Depcrate_core_build_steps_vendorimpl_804 {
() => {
// Module: crate::core::build_steps::vendor
// Provides: {"impl_804"}
// Dependencies: {}
impl Step for Vendor { type Output = VendorOutput ; const DEFAULT : bool = true ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . alias ("placeholder") . default_condition (true) } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (Vendor { sync_args : run . builder . config . cmd . vendor_sync_args () , versioned_dirs : run . builder . config . cmd . vendor_versioned_dirs () , root_dir : run . builder . src . clone () , output_dir : run . builder . src . join (VENDOR_DIR) , }) ; } # [doc = " Executes the vendoring process."] # [doc = ""] # [doc = " This function runs `cargo vendor` and ensures all required submodules"] # [doc = " are initialized before vendoring begins."] fn run (self , builder : & Builder < '_ >) -> Self :: Output { builder . info (& format ! ("Vendoring sources to {:?}" , self . root_dir)) ; let mut cmd = command (& builder . initial_cargo) ; cmd . arg ("vendor") ; if self . versioned_dirs { cmd . arg ("--versioned-dirs") ; } let to_vendor = default_paths_to_vendor (builder) ; for (_ , submodules) in & to_vendor { for submodule in submodules { builder . build . require_submodule (submodule , None) ; } } for (p , _) in & to_vendor { cmd . arg ("--sync") . arg (p) ; } for sync_arg in self . sync_args { cmd . arg ("--sync") . arg (sync_arg) ; } cmd . env ("RUSTC_BOOTSTRAP" , "1") ; cmd . env ("RUSTC" , & builder . initial_rustc) ; cmd . current_dir (self . root_dir) . arg (& self . output_dir) ; let config = cmd . run_capture_stdout (builder) ; VendorOutput { config : config . stdout () } } }
};
}
