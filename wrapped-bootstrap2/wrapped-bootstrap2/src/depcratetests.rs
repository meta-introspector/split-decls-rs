// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_bootstrap2_audit () { let mut auditor = Bootstrap2Auditor :: new () ; auditor . audit_core_functions () . unwrap () ; assert ! (! auditor . get_audit_log () . is_empty ()) ; } }
};
}
