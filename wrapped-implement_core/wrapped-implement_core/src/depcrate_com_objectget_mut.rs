// Generated macro for get_mut (function)
macro_rules! Depcrate_com_objectget_mut {
() => {
// Module: crate::com_object
// Provides: {"get_mut"}
// Dependencies: {}
# [test] fn get_mut () { let mut app : ComObject < MyApp > = MyApp :: new (42) ; assert_eq ! (app . get_x_direct () , 42) ; app . get_mut () . unwrap () . set_x (50) ; assert_eq ! (app . get_x_direct () , 50) ; let app2 = app . clone () ; assert ! (app . get_mut () . is_none ()) ; drop (app2) ; app . get_mut () . unwrap () . set_x (60) ; }
};
}
