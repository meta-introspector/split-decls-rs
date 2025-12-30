// Generated macro for phase_rustdoc (function)
macro_rules! Depcrate_phasesphase_rustdoc {
() => {
// Module: crate::phases
// Provides: {"phase_rustdoc"}
// Dependencies: {}
pub fn phase_rustdoc (mut args : impl Iterator < Item = String >) { let verbose = env :: var ("MIRI_VERBOSE") . map_or (0 , | verbose | verbose . parse () . expect ("verbosity flag must be an integer")) ; let rustdoc = env :: var ("MIRI_ORIG_RUSTDOC") . unwrap_or ("rustdoc" . to_string ()) ; let mut cmd = Command :: new (rustdoc) ; while let Some (arg) = args . next () { if arg == "--extern" { forward_patched_extern_arg (& mut args , & mut cmd) ; } else { cmd . arg (arg) ; } } if get_arg_flag_values ("--crate-type") . any (| crate_type | crate_type == "proc-macro") { eprintln ! ("Running doctests of `proc-macro` crates is not currently supported by Miri.") ; return ; } cmd . env ("MIRI_CALLED_FROM_RUSTDOC" , "1") ; cmd . arg ("-Zunstable-options") ; cmd . arg ("--sysroot") . arg (env :: var_os ("MIRI_SYSROOT") . unwrap ()) ; cmd . arg ("--cfg") . arg ("miri") ; let cargo_miri_path = env :: current_exe () . expect ("current executable path invalid") ; cmd . arg ("--test-builder") . arg (& cargo_miri_path) ; debug_cmd ("[cargo-miri rustdoc]" , verbose , & cmd) ; exec (cmd) }
};
}
