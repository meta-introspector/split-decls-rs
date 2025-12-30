// Generated macro for test_file (function)
macro_rules! Depcrate_fstest_file {
() => {
// Module: crate::fs
// Provides: {"test_file"}
// Dependencies: {}
fn test_file (path : & str) -> io :: Result < () > { let contents = "Hello, world!" ; eprint ! ("{path:15} : writing") ; fs :: write (path , contents) ? ; eprint ! (", reading") ; let read = fs :: read_to_string (path) ? ; assert_eq ! (contents , read) ; Ok (()) }
};
}
