// Generated macro for to_interface (function)
macro_rules! Depcrate_com_objectto_interface {
() => {
// Module: crate::com_object
// Provides: {"to_interface"}
// Dependencies: {}
# [test] fn to_interface () { let app = MyApp :: new (42) ; let tombstone = app . tombstone . clone () ; drop (app . to_interface :: < IUnknown > ()) ; let ifoo = app . to_interface :: < IFoo > () ; assert_eq ! (unsafe { ifoo . get_x () } , 42) ; assert ! (! tombstone . is_dead ()) ; drop (app) ; assert ! (! tombstone . is_dead ()) ; drop (ifoo) ; assert ! (tombstone . is_dead ()) ; }
};
}
