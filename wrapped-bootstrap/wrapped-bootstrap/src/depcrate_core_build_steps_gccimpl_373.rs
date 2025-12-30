// Generated macro for impl_373 (impl)
macro_rules! Depcrate_core_build_steps_gccimpl_373 {
() => {
// Module: crate::core::build_steps::gcc
// Provides: {"impl_373"}
// Dependencies: {}
impl GccOutput { # [doc = " Install the required libgccjit library file(s) to the specified `path`."] pub fn install_to (& self , builder : & Builder < '_ > , directory : & Path) { if builder . config . dry_run () { return ; } let mut target_filename = self . libgccjit . file_name () . unwrap () . to_str () . unwrap () . to_string () ; target_filename . push_str (".0") ; let actual_libgccjit_path = t ! (self . libgccjit . canonicalize () , format ! ("Cannot find libgccjit at {}" , self . libgccjit . display ())) ; let dst = directory . join (target_filename) ; builder . copy_link (& actual_libgccjit_path , & dst , FileType :: NativeLibrary) ; } }
};
}
