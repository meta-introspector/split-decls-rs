// Generated macro for casting (function)
macro_rules! Depcrate_com_objectcasting {
() => {
// Module: crate::com_object
// Provides: {"casting"}
// Dependencies: {}
# [test] fn casting () { let app : ComObject < MyApp > = MyApp :: new (42) ; let tombstone = app . tombstone . clone () ; let ifoo : IFoo = app . cast () . unwrap () ; assert_eq ! (unsafe { app . get_x () } , 42) ; assert ! (! tombstone . is_dead ()) ; drop (app) ; assert ! (! tombstone . is_dead ()) ; drop (ifoo) ; assert ! (tombstone . is_dead ()) ; }
};
}
