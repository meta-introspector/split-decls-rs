// Generated macro for tests (module)
macro_rules! Depcrate_reflogtests {
() => {
// Module: crate::reflog
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [test] fn smoke () { let (_td , repo) = crate :: test :: repo_init () ; let mut reflog = repo . reflog ("HEAD") . unwrap () ; assert_eq ! (reflog . iter () . len () , 1) ; reflog . write () . unwrap () ; let entry = reflog . iter () . next () . unwrap () ; assert ! (entry . message () . is_some ()) ; repo . reflog_rename ("HEAD" , "refs/heads/foo") . unwrap () ; repo . reflog_delete ("refs/heads/foo") . unwrap () ; } }
};
}
