// Generated macro for tests (module)
macro_rules! Depcrate_unametests {
() => {
// Module: crate::uname
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn uname_nonempty () { let val = uname (UnameField :: Sysname) . expect ("uname failed") ; assert ! (! val . is_empty ()) ; } }
};
}
