// Generated macro for tests (module)
macro_rules! Depcrate_util_idtests {
() => {
// Module: crate::util::id
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] # [cfg (feature = "string")] mod tests { use super :: * ; # [test] # [cfg (feature = "string")] fn from_cow_borrowed () { let cow = Cow :: Borrowed ("hello") ; let id = Id :: from (cow) ; assert_eq ! (id , Id :: from ("hello")) ; } # [test] # [cfg (feature = "string")] fn from_cow_owned () { let cow = Cow :: Owned ("world" . to_string ()) ; let id = Id :: from (cow) ; assert_eq ! (id , Id :: from ("world")) ; } }
};
}
