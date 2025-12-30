// Generated macro for impl_497 (impl)
macro_rules! Depcrate_traceimpl_497 {
() => {
// Module: crate::trace
// Provides: {"impl_497"}
// Dependencies: {}
impl ConnRef < '_ > { # [doc = " Test for auto-commit mode."] pub fn is_autocommit (& self) -> bool { unsafe { crate :: inner_connection :: get_autocommit (self . ptr) } } # [doc = " the path to the database file, if one exists and is known."] pub fn db_filename (& self) -> Option < & str > { unsafe { crate :: inner_connection :: db_filename (self . phantom , self . ptr , MAIN_DB) } } }
};
}
