// Generated macro for tests (module)
macro_rules! Depcrate_pathspectests {
() => {
// Module: crate::pathspec
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: Pathspec ; use crate :: PathspecFlags ; use std :: fs :: File ; use std :: path :: Path ; # [test] fn smoke () { let ps = Pathspec :: new (["a"] . iter ()) . unwrap () ; assert ! (ps . matches_path (Path :: new ("a") , PathspecFlags :: DEFAULT)) ; assert ! (ps . matches_path (Path :: new ("a/b") , PathspecFlags :: DEFAULT)) ; assert ! (! ps . matches_path (Path :: new ("b") , PathspecFlags :: DEFAULT)) ; assert ! (! ps . matches_path (Path :: new ("ab/c") , PathspecFlags :: DEFAULT)) ; let (td , repo) = crate :: test :: repo_init () ; let list = ps . match_workdir (& repo , PathspecFlags :: DEFAULT) . unwrap () ; assert_eq ! (list . entries () . len () , 0) ; assert_eq ! (list . diff_entries () . len () , 0) ; assert_eq ! (list . failed_entries () . len () , 0) ; File :: create (& td . path () . join ("a")) . unwrap () ; let list = ps . match_workdir (& repo , crate :: PathspecFlags :: FIND_FAILURES) . unwrap () ; assert_eq ! (list . entries () . len () , 1) ; assert_eq ! (list . entries () . next () , Some ("a" . as_bytes ())) ; } }
};
}
