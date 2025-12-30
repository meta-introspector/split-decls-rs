// Generated macro for ConflictAction (enum)
macro_rules! Depcrate_sessionConflictAction {
() => {
// Module: crate::session
// Provides: {"ConflictAction"}
// Dependencies: {}
# [doc = " Constants returned by the conflict handler"] # [doc = " See [here](https://sqlite.org/session.html#SQLITE_CHANGESET_ABORT) for details."] # [allow (missing_docs)] # [repr (i32)] # [derive (Debug , PartialEq , Eq)] # [non_exhaustive] pub enum ConflictAction { SQLITE_CHANGESET_OMIT = ffi :: SQLITE_CHANGESET_OMIT , SQLITE_CHANGESET_REPLACE = ffi :: SQLITE_CHANGESET_REPLACE , SQLITE_CHANGESET_ABORT = ffi :: SQLITE_CHANGESET_ABORT , }
};
}
