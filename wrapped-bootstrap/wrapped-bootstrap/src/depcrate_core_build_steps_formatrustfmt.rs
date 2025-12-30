// Generated macro for rustfmt (function)
macro_rules! Depcrate_core_build_steps_formatrustfmt {
() => {
// Module: crate::core::build_steps::format
// Provides: {"rustfmt"}
// Dependencies: {}
fn rustfmt (src : & Path , rustfmt : & Path , paths : & [PathBuf] , check : bool ,) -> impl FnMut (bool) -> RustfmtStatus + use < > { let mut cmd = Command :: new (rustfmt) ; cmd . arg ("--config-path") . arg (src . canonicalize () . unwrap ()) ; cmd . arg ("--edition") . arg ("2024") ; cmd . arg ("--unstable-features") ; cmd . arg ("--skip-children") ; if check { cmd . arg ("--check") ; } cmd . args (paths) ; let mut cmd = cmd . spawn () . expect ("running rustfmt") ; move | block : bool | -> RustfmtStatus { let status = if ! block { match cmd . try_wait () { Ok (Some (status)) => Ok (status) , Ok (None) => return RustfmtStatus :: InProgress , Err (err) => Err (err) , } } else { cmd . wait () } ; if status . unwrap () . success () { RustfmtStatus :: Ok } else { RustfmtStatus :: Failed } } }
};
}
