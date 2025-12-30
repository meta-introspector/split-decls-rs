// Generated macro for tests (module)
macro_rules! Depcrate_builder_styled_strtests {
() => {
// Module: crate::builder::styled_str
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn from_cow_borrowed () { let cow = Cow :: Borrowed ("hello") ; let styled = StyledStr :: from (cow) ; assert_eq ! (styled , StyledStr :: from ("hello")) ; } # [test] fn from_cow_owned () { let cow = Cow :: Owned ("world" . to_string ()) ; let styled = StyledStr :: from (cow) ; assert_eq ! (styled , StyledStr :: from ("world")) ; } }
};
}
