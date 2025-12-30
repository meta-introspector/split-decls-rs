// Generated macro for Vendor (struct)
macro_rules! Depcrate_core_build_steps_vendorVendor {
() => {
// Module: crate::core::build_steps::vendor
// Provides: {"Vendor"}
// Dependencies: {}
# [doc = " Defines the vendoring step in the bootstrap process."] # [doc = ""] # [doc = " This step executes `cargo vendor` to collect all dependencies"] # [doc = " and store them in the `<src>/<VENDOR_DIR>` directory."] # [derive (Debug , Clone , Hash , PartialEq , Eq)] pub (crate) struct Vendor { # [doc = " Additional paths to synchronize during vendoring."] pub (crate) sync_args : Vec < PathBuf > , # [doc = " Determines whether vendored dependencies use versioned directories."] pub (crate) versioned_dirs : bool , # [doc = " The root directory of the source code."] pub (crate) root_dir : PathBuf , # [doc = " The target directory for storing vendored dependencies."] pub (crate) output_dir : PathBuf , }
};
}
