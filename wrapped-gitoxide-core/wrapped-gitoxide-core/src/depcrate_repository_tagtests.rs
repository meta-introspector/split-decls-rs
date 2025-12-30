// Generated macro for tests (module)
macro_rules! Depcrate_repository_tagtests {
() => {
// Module: crate::repository::tag
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use std :: cmp :: Ordering ; # [test] fn sorts_versions_correctly () { let mut actual = vec ! ["v2.0.0" , "v1.10.0" , "v1.2.1" , "v1.0.0-beta" , "v1.2" , "v0.10.0" , "v0.9.0" , "v1.2.0" , "v0.1.a" , "v0.1.0" , "v10.0.0" , "1.0.0" , "v1.0.0-alpha" , "v1.0.0" ,] ; actual . sort_by (| & a , & b | Version :: parse (a . into ()) . cmp (& Version :: parse (b . into ()))) ; let expected = ["v0.1.0" , "v0.1.a" , "v0.9.0" , "v0.10.0" , "v1.0.0" , "v1.0.0-alpha" , "v1.0.0-beta" , "v1.2" , "v1.2.0" , "v1.2.1" , "v1.10.0" , "v2.0.0" , "v10.0.0" , "1.0.0" ,] ; assert_eq ! (actual , expected) ; } # [test] fn sorts_versions_with_different_lengths_correctly () { let v1 = Version :: parse ("v1.0" . into ()) ; let v2 = Version :: parse ("v1.0.1" . into ()) ; assert_eq ! (v1 . cmp (& v2) , Ordering :: Less) ; assert_eq ! (v2 . cmp (& v1) , Ordering :: Greater) ; } }
};
}
