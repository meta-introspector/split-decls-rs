// Generated macro for urls_with_schemes_other_than_mysql_are_errors (function)
macro_rules! Depcrate_mysql_connection_urlurls_with_schemes_other_than_mysql_are_errors {
() => {
// Module: crate::mysql::connection::url
// Provides: {"urls_with_schemes_other_than_mysql_are_errors"}
// Dependencies: {}
# [test] fn urls_with_schemes_other_than_mysql_are_errors () { assert ! (ConnectionOptions :: parse ("postgres://localhost") . is_err ()) ; assert ! (ConnectionOptions :: parse ("http://localhost") . is_err ()) ; assert ! (ConnectionOptions :: parse ("file:///tmp/mysql.sock") . is_err ()) ; assert ! (ConnectionOptions :: parse ("socket:///tmp/mysql.sock") . is_err ()) ; assert ! (ConnectionOptions :: parse ("mysql://localhost?database=somedb") . is_err ()) ; assert ! (ConnectionOptions :: parse ("mysql://localhost") . is_ok ()) ; }
};
}
