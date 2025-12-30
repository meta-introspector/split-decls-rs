// Generated macro for check (function)
macro_rules! Depcrate_errorcheck {
() => {
// Module: crate::error
// Provides: {"check"}
// Dependencies: {}
pub fn check (code : c_int) -> Result < () > { if code != ffi :: SQLITE_OK { Err (error_from_sqlite_code (code , None)) } else { Ok (()) } }
};
}
