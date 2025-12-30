// Generated macro for forward_patched_extern_arg (function)
macro_rules! Depcrate_phasesforward_patched_extern_arg {
() => {
// Module: crate::phases
// Provides: {"forward_patched_extern_arg"}
// Dependencies: {}
fn forward_patched_extern_arg (args : & mut impl Iterator < Item = String > , cmd : & mut Command) { cmd . arg ("--extern") ; let path = args . next () . expect ("`--extern` should be followed by a filename") ; if let Some (lib) = path . strip_suffix (".rlib") { cmd . arg (format ! ("{lib}.rmeta")) ; } else { cmd . arg (path) ; } }
};
}
