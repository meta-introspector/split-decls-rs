// Generated macro for impl_24 (impl)
macro_rules! Depcrateimpl_24 {
() => {
// Module: crate
// Provides: {"impl_24"}
// Dependencies: {}
impl RustfmtRunner { fn get_binary_version (& self) -> Result < String , CheckDiffError > { let Ok (command) = Command :: new (& self . binary_path) . env ("LD_LIBRARY_PATH" , & self . ld_library_path) . args (["--version"]) . output () else { return Err (CheckDiffError :: FailedBinaryVersioning (self . binary_path . clone () ,)) ; } ; let binary_version = std :: str :: from_utf8 (& command . stdout) ? . trim () ; return Ok (binary_version . to_string ()) ; } }
};
}
