// Generated macro for as_interface (function)
macro_rules! Depcrate_com_objectas_interface {
() => {
// Module: crate::com_object
// Provides: {"as_interface"}
// Dependencies: {}
# [test] fn as_interface () { let app = MyApp :: new (42) ; let tombstone = app . tombstone . clone () ; let _ = app . as_interface :: < IUnknown > () ; let ifoo = app . as_interface :: < IFoo > () ; assert_eq ! (unsafe { ifoo . get_x () } , 42) ; assert ! (! tombstone . is_dead ()) ; drop (app) ; assert ! (tombstone . is_dead ()) ; }
};
}
