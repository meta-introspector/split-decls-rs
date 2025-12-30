// Generated macro for update_output (function)
macro_rules! Depcrate_transforms_threads_testsupdate_output {
() => {
// Module: crate::transforms::threads::tests
// Provides: {"update_output"}
// Dependencies: {}
fn update_output (path : & Path , output : & str) -> Result < () > { let contents = fs :: read_to_string (path) ? ; let start = contents . find ("(; CHECK-ALL:") . unwrap_or (contents . len ()) ; let mut new_output = String :: new () ; for line in output . lines () { new_output . push_str (line) ; new_output . push ('\n') ; } let new = format ! ("{}\n\n(; CHECK-ALL:\n{}\n;)\n" , contents [.. start] . trim () , new_output . trim_end ()) ; fs :: write (path , new) ? ; Ok (()) }
};
}
