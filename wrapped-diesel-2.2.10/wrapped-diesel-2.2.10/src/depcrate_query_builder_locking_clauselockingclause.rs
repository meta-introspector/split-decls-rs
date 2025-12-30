// Generated macro for LockingClause (struct)
macro_rules! Depcrate_query_builder_locking_clauseLockingClause {
() => {
// Module: crate::query_builder::locking_clause
// Provides: {"LockingClause"}
// Dependencies: {}
# [derive (Debug , Clone , Copy , QueryId)] pub struct LockingClause < LockMode = ForUpdate , Modifier = NoModifier > { pub (crate) lock_mode : LockMode , modifier : Modifier , }
};
}
