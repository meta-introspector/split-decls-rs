// Generated macro for unix_socket_tests (function)
macro_rules! Depcrate_mysql_connection_urlunix_socket_tests {
() => {
// Module: crate::mysql::connection::url
// Provides: {"unix_socket_tests"}
// Dependencies: {}
# [test] fn unix_socket_tests () { let unix_socket = "/var/run/mysqld.sock" ; let username = "foo" ; let password = "bar" ; let db_url = format ! ("mysql://{username}:{password}@localhost?unix_socket={unix_socket}" ,) ; let conn_opts = ConnectionOptions :: parse (db_url . as_str ()) . unwrap () ; let cstring = | s | CString :: new (s) . unwrap () ; assert_eq ! (None , conn_opts . host) ; assert_eq ! (None , conn_opts . port) ; assert_eq ! (cstring (username) , conn_opts . user) ; assert_eq ! (cstring (password) , conn_opts . password . unwrap ()) ; assert_eq ! (CString :: new (unix_socket) . unwrap () , conn_opts . unix_socket . unwrap ()) ; }
};
}
