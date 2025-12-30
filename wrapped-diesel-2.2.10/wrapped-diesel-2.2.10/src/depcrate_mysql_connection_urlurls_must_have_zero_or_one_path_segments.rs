// Generated macro for urls_must_have_zero_or_one_path_segments (function)
macro_rules! Depcrate_mysql_connection_urlurls_must_have_zero_or_one_path_segments {
() => {
// Module: crate::mysql::connection::url
// Provides: {"urls_must_have_zero_or_one_path_segments"}
// Dependencies: {}
# [test] fn urls_must_have_zero_or_one_path_segments () { assert ! (ConnectionOptions :: parse ("mysql://localhost/foo/bar") . is_err ()) ; assert ! (ConnectionOptions :: parse ("mysql://localhost/foo") . is_ok ()) ; }
};
}
