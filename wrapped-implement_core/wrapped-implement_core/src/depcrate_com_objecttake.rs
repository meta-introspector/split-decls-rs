// Generated macro for take (function)
macro_rules! Depcrate_com_objecttake {
() => {
// Module: crate::com_object
// Provides: {"take"}
// Dependencies: {}
# [test] fn take () { let app : ComObject < MyApp > = MyApp :: new (42) ; let tombstone = app . tombstone . clone () ; let app2 = app . clone () ; let app2_rejected : ComObject < MyApp > = match app2 . take () { Ok (_unexpected) => panic ! ("take() should have failed") , Err (e) => e , } ; drop (app2_rejected) ; match app . take () { Ok (unwrapped_app) => { assert ! (! tombstone . is_dead ()) ; assert_eq ! (unwrapped_app . x , 42) ; drop (unwrapped_app) ; assert ! (tombstone . is_dead ()) ; } Err (_unexpected) => { panic ! ("take() should have succeeded") ; } } }
};
}
