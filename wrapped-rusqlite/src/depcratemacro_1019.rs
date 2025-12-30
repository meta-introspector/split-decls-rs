// Generated macro for macro_1019 (macro)
macro_rules! Depcratemacro_1019 {
() => {
// Module: crate
// Provides: {"macro_1019"}
// Dependencies: {}
bitflags :: bitflags ! { # [doc = " Prepare flags. See"] # [doc = " [sqlite3_prepare_v3](https://sqlite.org/c3ref/c_prepare_normalize.html) for details."] # [derive (Clone , Copy , Debug , Default , Eq , Hash , PartialEq)] # [repr (C)] pub struct PrepFlags : c_uint { # [doc = " A hint to the query planner that the prepared statement will be retained for a long time and probably reused many times."] const SQLITE_PREPARE_PERSISTENT = 0x01 ; # [doc = " Causes the SQL compiler to return an error (error code SQLITE_ERROR) if the statement uses any virtual tables."] const SQLITE_PREPARE_NO_VTAB = 0x04 ; # [doc = " Prevents SQL compiler errors from being sent to the error log."] const SQLITE_PREPARE_DONT_LOG = 0x10 ; } }
};
}
