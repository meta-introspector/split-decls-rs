// Generated macro for create_fnmut (function)
macro_rules! Depcrate_factorycreate_fnmut {
() => {
// Module: crate::factory
// Provides: {"create_fnmut"}
// Dependencies: {}
# [test] fn create_fnmut () { let f = Factory :: new_fnmut :: < () > () ; let mut move_me = 5u32 ; let m = f . method ("test" , () , move | m | { move_me += 1 ; Ok (vec ! (m . msg . method_return () . append1 (& move_me))) }) ; assert_eq ! (&** m . get_name () , "test") ; }
};
}
