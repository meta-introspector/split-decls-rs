// Generated macro for tests (module)
macro_rules! Depcrate_ext_vectests {
() => {
// Module: crate::ext_vec
// Provides: {"tests"}
// Dependencies: {}
# [cfg (all (test , feature = "std"))] mod tests { use alloc :: { vec , vec :: Vec } ; use crate :: ext_vec :: ByteVec ; # [test] fn insert () { let mut s = vec ! [] ; s . insert_str (0 , "foo") ; assert_eq ! (s , "foo" . as_bytes ()) ; let mut s = Vec :: from ("a") ; s . insert_str (0 , "foo") ; assert_eq ! (s , "fooa" . as_bytes ()) ; let mut s = Vec :: from ("a") ; s . insert_str (1 , "foo") ; assert_eq ! (s , "afoo" . as_bytes ()) ; let mut s = Vec :: from ("foobar") ; s . insert_str (3 , "quux") ; assert_eq ! (s , "fooquuxbar" . as_bytes ()) ; let mut s = Vec :: from ("foobar") ; s . insert_str (3 , "x") ; assert_eq ! (s , "fooxbar" . as_bytes ()) ; let mut s = Vec :: from ("foobar") ; s . insert_str (0 , "x") ; assert_eq ! (s , "xfoobar" . as_bytes ()) ; let mut s = Vec :: from ("foobar") ; s . insert_str (6 , "x") ; assert_eq ! (s , "foobarx" . as_bytes ()) ; let mut s = Vec :: from ("foobar") ; s . insert_str (3 , "quuxbazquux") ; assert_eq ! (s , "fooquuxbazquuxbar" . as_bytes ()) ; } # [test] # [should_panic] fn insert_fail1 () { let mut s = vec ! [] ; s . insert_str (1 , "foo") ; } # [test] # [should_panic] fn insert_fail2 () { let mut s = Vec :: from ("a") ; s . insert_str (2 , "foo") ; } # [test] # [should_panic] fn insert_fail3 () { let mut s = Vec :: from ("foobar") ; s . insert_str (7 , "foo") ; } }
};
}
