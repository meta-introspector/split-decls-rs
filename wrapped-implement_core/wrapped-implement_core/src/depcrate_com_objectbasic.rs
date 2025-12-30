// Generated macro for basic (function)
macro_rules! Depcrate_com_objectbasic {
() => {
// Module: crate::com_object
// Provides: {"basic"}
// Dependencies: {}
# [test] fn basic () { let app : ComObject < MyApp > = MyApp :: new (42) ; let iunknown : IUnknown = app . cast () . unwrap () ; let ifoo : IFoo = app . cast () . unwrap () ; assert_eq ! (unsafe { ifoo . get_x () } , 42) ; let tombstone = app . tombstone . clone () ; assert ! (! tombstone . is_dead ()) ; drop (app) ; assert ! (! tombstone . is_dead ()) ; drop (iunknown) ; assert ! (! tombstone . is_dead ()) ; drop (ifoo) ; assert ! (tombstone . is_dead ()) ; }
};
}
