// Generated macro for tests (module)
macro_rules! Depcrate_eventtests {
() => {
// Module: crate::event
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use std :: borrow :: Cow ; # [test] fn parse_event_id_no_args () { let (label , args) = Event :: parse_event_id (Cow :: from ("foo")) ; assert_eq ! (label , "foo") ; assert ! (args . is_empty ()) ; } # [test] fn parse_event_id_with_control_char () { let (label , args) = Event :: parse_event_id (Cow :: from ("foo\x1b")) ; assert_eq ! (label , "<parse error>") ; assert ! (args . is_empty ()) ; } # [test] fn parse_event_id_one_arg () { let (label , args) = Event :: parse_event_id (Cow :: from ("foo\x1emy_arg")) ; assert_eq ! (label , "foo") ; assert_eq ! (args , vec ! [Cow :: from ("my_arg")]) ; } # [test] fn parse_event_id_n_args () { let (label , args) = Event :: parse_event_id (Cow :: from ("foo\x1earg1\x1earg2\x1earg3")) ; assert_eq ! (label , "foo") ; assert_eq ! (args , vec ! [Cow :: from ("arg1") , Cow :: from ("arg2") , Cow :: from ("arg3")]) ; } # [test] fn parse_event_id_args_with_whitespace () { let (label , args) = Event :: parse_event_id (Cow :: from ("foo\x1earg\n1\x1earg\t2\x1earg 3")) ; assert_eq ! (label , "foo") ; assert_eq ! (args , vec ! [Cow :: from ("arg\n1") , Cow :: from ("arg\t2") , Cow :: from ("arg 3")]) ; } # [test] fn parse_event_id_args_with_control_char () { let (label , args) = Event :: parse_event_id (Cow :: from ("foo\x1earg\x1b1")) ; assert_eq ! (label , "foo") ; assert ! (args . is_empty ()) ; } }
};
}
