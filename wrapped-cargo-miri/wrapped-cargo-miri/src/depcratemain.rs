// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let mut args = env :: args () ; args . next () . unwrap () ; if env :: var_os ("MIRI_CALLED_FROM_SETUP") . is_some () { phase_rustc (args , RustcPhase :: Setup) ; return ; } let Some (first) = args . next () else { show_error ! ("`cargo-miri` called without first argument; please only invoke this binary through `cargo miri`") } ; if env :: var_os ("MIRI_CALLED_FROM_RUSTDOC") . is_some () { match first . as_str () { "runner" => phase_runner (args , RunnerPhase :: Rustdoc) , flag if flag . starts_with ("--") || flag . starts_with ("@") => { phase_rustc (iter :: once (first) . chain (args) , RustcPhase :: Rustdoc) ; } _ => { show_error ! ("`cargo-miri` failed to recognize which phase of the build process this is, please report a bug.\n\
                    We are inside MIRI_CALLED_FROM_RUSTDOC.\n\
                    The command-line arguments were: {:#?}" , Vec :: from_iter (env :: args ()) ,) ; } } return ; } match first . as_str () { "miri" => phase_cargo_miri (args) , "runner" => phase_runner (args , RunnerPhase :: Cargo) , arg if arg == env :: var ("RUSTC") . unwrap_or_else (| _ | { show_error ! ("`cargo-miri` called without RUSTC set; please only invoke this binary through `cargo miri`") }) => { phase_rustc (args , RustcPhase :: Build) } _ if looks_like_rustdoc () => { phase_rustdoc (iter :: once (first) . chain (args)) ; } _ => { show_error ! ("`cargo-miri` failed to recognize which phase of the build process this is, please report a bug.\nThe command-line arguments were: {:#?}" , Vec :: from_iter (env :: args ()) ,) ; } } }
};
}
