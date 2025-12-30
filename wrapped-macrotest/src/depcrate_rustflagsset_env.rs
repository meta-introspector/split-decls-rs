// Generated macro for set_env (function)
macro_rules! Depcrate_rustflagsset_env {
() => {
// Module: crate::rustflags
// Provides: {"set_env"}
// Dependencies: {}
pub fn set_env (cmd : & mut Command) { let (key , mut val , separator) = match env :: var_os (CARGO_ENCODED_RUSTFLAGS) { Some (val) => (CARGO_ENCODED_RUSTFLAGS , val , "\x1f") , None => match env :: var_os (RUSTFLAGS) { Some (val) => (RUSTFLAGS , val , " ") , None => return , } , } ; for flag in make_vec () { if ! val . is_empty () { val . push (separator) ; } val . push (flag) ; } cmd . env (key , val) ; }
};
}
