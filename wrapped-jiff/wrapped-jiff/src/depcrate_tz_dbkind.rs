// Generated macro for Kind (enum)
macro_rules! Depcrate_tz_dbKind {
() => {
// Module: crate::tz::db
// Provides: {"Kind"}
// Dependencies: {}
# [derive (Debug)] # [cfg_attr (not (feature = "alloc") , derive (Clone))] enum Kind { ZoneInfo (zoneinfo :: Database) , Concatenated (concatenated :: Database) , Bundled (bundled :: Database) , }
};
}
