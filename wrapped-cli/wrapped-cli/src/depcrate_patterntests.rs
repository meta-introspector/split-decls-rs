// Generated macro for tests (module)
macro_rules! Depcrate_patterntests {
() => {
// Module: crate::pattern
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn bytes () { let pat = b"abc\xFFxyz" ; let err = pattern_from_bytes (pat) . unwrap_err () ; assert_eq ! (3 , err . valid_up_to ()) ; } # [test] # [cfg (unix)] fn os () { use std :: ffi :: OsStr ; use std :: os :: unix :: ffi :: OsStrExt ; let pat = OsStr :: from_bytes (b"abc\xFFxyz") ; let err = pattern_from_os (pat) . unwrap_err () ; assert_eq ! (3 , err . valid_up_to ()) ; } }
};
}
