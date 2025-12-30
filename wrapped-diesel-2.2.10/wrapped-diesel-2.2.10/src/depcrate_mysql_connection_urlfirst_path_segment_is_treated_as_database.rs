// Generated macro for first_path_segment_is_treated_as_database (function)
macro_rules! Depcrate_mysql_connection_urlfirst_path_segment_is_treated_as_database {
() => {
// Module: crate::mysql::connection::url
// Provides: {"first_path_segment_is_treated_as_database"}
// Dependencies: {}
# [test] fn first_path_segment_is_treated_as_database () { let foo_cstr = CString :: new ("foo") . unwrap () ; let bar_cstr = CString :: new ("bar") . unwrap () ; assert_eq ! (Some (&* foo_cstr) , ConnectionOptions :: parse ("mysql://localhost/foo") . unwrap () . database ()) ; assert_eq ! (Some (&* bar_cstr) , ConnectionOptions :: parse ("mysql://localhost/bar") . unwrap () . database ()) ; assert_eq ! (None , ConnectionOptions :: parse ("mysql://localhost") . unwrap () . database ()) ; }
};
}
