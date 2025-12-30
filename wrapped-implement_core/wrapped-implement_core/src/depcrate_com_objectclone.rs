// Generated macro for clone (function)
macro_rules! Depcrate_com_objectclone {
() => {
// Module: crate::com_object
// Provides: {"clone"}
// Dependencies: {}
# [test] fn clone () { let app : ComObject < MyApp > = MyApp :: new (42) ; let ifoo : IFoo = app . cast () . unwrap () ; let ifoo2 = ifoo . clone () ; drop (ifoo) ; drop (app) ; assert_eq ! (unsafe { ifoo2 . get_x () } , 42) ; }
};
}
