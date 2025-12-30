// Generated macro for tests (module)
macro_rules! Depcrate_terminaltests {
() => {
// Module: crate::terminal
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: { io :: stdout , thread , time } ; use crate :: execute ; use super :: * ; # [test] # [ignore] fn test_resize_ansi () { let (width , height) = size () . unwrap () ; execute ! (stdout () , SetSize (35 , 35)) . unwrap () ; thread :: sleep (time :: Duration :: from_millis (30)) ; assert_eq ! ((35 , 35) , size () . unwrap ()) ; execute ! (stdout () , SetSize (width , height)) . unwrap () ; thread :: sleep (time :: Duration :: from_millis (30)) ; assert_eq ! ((width , height) , size () . unwrap ()) ; } # [test] fn test_raw_mode () { assert ! (! is_raw_mode_enabled () . unwrap ()) ; if enable_raw_mode () . is_err () { return ; } assert ! (is_raw_mode_enabled () . unwrap ()) ; enable_raw_mode () . unwrap () ; assert ! (is_raw_mode_enabled () . unwrap ()) ; disable_raw_mode () . unwrap () ; assert ! (! is_raw_mode_enabled () . unwrap ()) ; } }
};
}
