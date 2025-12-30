// Generated macro for cp_rustc_component_to_ci_sysroot (function)
macro_rules! Depcrate_core_build_steps_compilecp_rustc_component_to_ci_sysroot {
() => {
// Module: crate::core::build_steps::compile
// Provides: {"cp_rustc_component_to_ci_sysroot"}
// Dependencies: {}
fn cp_rustc_component_to_ci_sysroot (builder : & Builder < '_ > , sysroot : & Path , contents : Vec < String >) { let ci_rustc_dir = builder . config . ci_rustc_dir () ; for file in contents { let src = ci_rustc_dir . join (& file) ; let dst = sysroot . join (file) ; if src . is_dir () { t ! (fs :: create_dir_all (dst)) ; } else { builder . copy_link (& src , & dst , FileType :: Regular) ; } } }
};
}
