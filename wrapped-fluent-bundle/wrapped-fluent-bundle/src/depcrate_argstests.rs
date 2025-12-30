// Generated macro for tests (module)
macro_rules! Depcrate_argstests {
() => {
// Module: crate::args
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn replace_existing_arguments () { let mut args = FluentArgs :: new () ; args . set ("name" , "John") ; args . set ("emailCount" , 5) ; assert_eq ! (args . 0 . len () , 2) ; assert_eq ! (args . get ("name") , Some (& FluentValue :: String (Cow :: Borrowed ("John")))) ; assert_eq ! (args . get ("emailCount") , Some (& FluentValue :: try_number ("5"))) ; args . set ("name" , "Jane") ; args . set ("emailCount" , 7) ; assert_eq ! (args . 0 . len () , 2) ; assert_eq ! (args . get ("name") , Some (& FluentValue :: String (Cow :: Borrowed ("Jane")))) ; assert_eq ! (args . get ("emailCount") , Some (& FluentValue :: try_number ("7"))) ; } }
};
}
