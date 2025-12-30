// Generated macro for to_object (function)
macro_rules! Depcrate_com_objectto_object {
() => {
// Module: crate::com_object
// Provides: {"to_object"}
// Dependencies: {}
# [test] fn to_object () { let app = MyApp :: new (42) ; let tombstone = app . tombstone . clone () ; let app_outer : & MyApp_Impl = & app ; let second_app = app_outer . to_object () ; assert ! (! tombstone . is_dead ()) ; assert_eq ! (second_app . signature , APP_SIGNATURE) ; println ! ("x = {}" , unsafe { second_app . get_x () }) ; drop (second_app) ; assert ! (! tombstone . is_dead ()) ; drop (app) ; assert ! (tombstone . is_dead ()) ; }
};
}
