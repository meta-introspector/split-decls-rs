// Generated macro for into_interface (function)
macro_rules! Depcrate_com_objectinto_interface {
() => {
// Module: crate::com_object
// Provides: {"into_interface"}
// Dependencies: {}
# [test] fn into_interface () { let app = MyApp :: new (42) ; let tombstone = app . tombstone . clone () ; let ifoo = app . into_interface :: < IFoo > () ; assert_eq ! (unsafe { ifoo . get_x () } , 42) ; assert ! (! tombstone . is_dead ()) ; drop (ifoo) ; assert ! (tombstone . is_dead ()) ; }
};
}
