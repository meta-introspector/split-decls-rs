// Generated macro for tests (module)
macro_rules! Depcrate_branchtests {
() => {
// Module: crate::branch
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: { Branch , BranchType } ; # [test] fn smoke () { let (_td , repo) = crate :: test :: repo_init () ; let head = repo . head () . unwrap () ; let target = head . target () . unwrap () ; let commit = repo . find_commit (target) . unwrap () ; let mut b1 = repo . branch ("foo" , & commit , false) . unwrap () ; assert ! (! b1 . is_head ()) ; repo . branch ("foo2" , & commit , false) . unwrap () ; assert_eq ! (repo . branches (None) . unwrap () . count () , 3) ; repo . find_branch ("foo" , BranchType :: Local) . unwrap () ; let mut b1 = b1 . rename ("bar" , false) . unwrap () ; assert_eq ! (b1 . name () . unwrap () , Some ("bar")) ; assert ! (b1 . upstream () . is_err ()) ; b1 . set_upstream (Some ("main")) . unwrap () ; b1 . upstream () . unwrap () ; b1 . set_upstream (None) . unwrap () ; b1 . delete () . unwrap () ; } # [test] fn name_is_valid () { assert ! (Branch :: name_is_valid ("foo") . unwrap ()) ; assert ! (! Branch :: name_is_valid ("") . unwrap ()) ; assert ! (! Branch :: name_is_valid ("with spaces") . unwrap ()) ; assert ! (! Branch :: name_is_valid ("~tilde") . unwrap ()) ; } }
};
}
