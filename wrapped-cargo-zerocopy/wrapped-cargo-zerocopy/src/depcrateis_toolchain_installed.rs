// Generated macro for is_toolchain_installed (function)
macro_rules! Depcrateis_toolchain_installed {
() => {
// Module: crate
// Provides: {"is_toolchain_installed"}
// Dependencies: {}
fn is_toolchain_installed (versions : & Versions , name : & str) -> Result < bool , Error > { let version = versions . get (name) ? ; let output = rustup (["run" , version , "cargo" , "version"] , None) . output () . unwrap () ; if output . status . success () { let output = rustup ([& format ! ("+{version}") , "component" , "list"] , None) . output_or_exit () ; let stdout = String :: from_utf8 (output . stdout) . unwrap () ; Ok (stdout . contains ("rust-src (installed)")) } else { Ok (false) } }
};
}
