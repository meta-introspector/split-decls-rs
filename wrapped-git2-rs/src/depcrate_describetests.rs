// Generated macro for tests (module)
macro_rules! Depcrate_describetests {
() => {
// Module: crate::describe
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: DescribeOptions ; # [test] fn smoke () { let (_td , repo) = crate :: test :: repo_init () ; let head = t ! (repo . head ()) . target () . unwrap () ; let d = t ! (repo . describe (DescribeOptions :: new () . show_commit_oid_as_fallback (true))) ; let id = head . to_string () ; assert_eq ! (t ! (d . format (None)) , & id [.. 7]) ; let obj = t ! (repo . find_object (head , None)) ; let sig = t ! (repo . signature ()) ; t ! (repo . tag ("foo" , & obj , & sig , "message" , true)) ; let d = t ! (repo . describe (& DescribeOptions :: new ())) ; assert_eq ! (t ! (d . format (None)) , "foo") ; let d = t ! (obj . describe (& DescribeOptions :: new ())) ; assert_eq ! (t ! (d . format (None)) , "foo") ; } }
};
}
