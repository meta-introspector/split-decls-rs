// Generated macro for is_c_like (function)
macro_rules! Depcrate_backends_shared_posixis_c_like {
() => {
// Module: crate::backends::shared::posix
// Provides: {"is_c_like"}
// Dependencies: {}
# [inline] fn is_c_like (raw : & str) -> bool { let s = raw . trim () ; if s . is_empty () { return true ; } let up = s . to_ascii_uppercase () ; let base = up . split ('.') . next () . unwrap_or (& up) ; let base = base . split ('@') . next () . unwrap_or (base) ; base == "C" || base == "POSIX" }
};
}
