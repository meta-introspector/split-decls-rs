// Generated macro for tests (module)
macro_rules! Depcrate_tagforeachtests {
() => {
// Module: crate::tagforeach
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [test] fn smoke () { let (_td , repo) = crate :: test :: repo_init () ; let head = repo . head () . unwrap () ; let id = head . target () . unwrap () ; assert ! (repo . find_tag (id) . is_err ()) ; let obj = repo . find_object (id , None) . unwrap () ; let sig = repo . signature () . unwrap () ; let tag_id = repo . tag ("foo" , & obj , & sig , "msg" , false) . unwrap () ; let mut tags = Vec :: new () ; repo . tag_foreach (| id , name | { tags . push ((id , String :: from_utf8 (name . into ()) . unwrap ())) ; true }) . unwrap () ; assert_eq ! (tags [0] . 0 , tag_id) ; assert_eq ! (tags [0] . 1 , "refs/tags/foo") ; } }
};
}
