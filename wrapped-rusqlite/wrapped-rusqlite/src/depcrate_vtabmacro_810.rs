// Generated macro for macro_810 (macro)
macro_rules! Depcrate_vtabmacro_810 {
() => {
// Module: crate::vtab
// Provides: {"macro_810"}
// Dependencies: {}
bitflags :: bitflags ! { # [doc = " Virtual table scan flags"] # [doc = " See [Function Flags](https://sqlite.org/c3ref/c_index_scan_unique.html) for details."] # [repr (C)] # [derive (Copy , Clone , Debug)] pub struct IndexFlags : c_int { # [doc = " Default"] const NONE = 0 ; # [doc = " Scan visits at most 1 row."] const SQLITE_INDEX_SCAN_UNIQUE = ffi :: SQLITE_INDEX_SCAN_UNIQUE ; # [doc = " Display idxNum as hex in EXPLAIN QUERY PLAN"] const SQLITE_INDEX_SCAN_HEX = 0x0000_0002 ; } }
};
}
