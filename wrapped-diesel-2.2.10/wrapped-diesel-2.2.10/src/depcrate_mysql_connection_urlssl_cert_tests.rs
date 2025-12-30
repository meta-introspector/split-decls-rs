// Generated macro for ssl_cert_tests (function)
macro_rules! Depcrate_mysql_connection_urlssl_cert_tests {
() => {
// Module: crate::mysql::connection::url
// Provides: {"ssl_cert_tests"}
// Dependencies: {}
# [test] fn ssl_cert_tests () { let ssl_cert = "/etc/ssl/certs/client-cert.crt" ; let username = "foo" ; let password = "bar" ; let db_url = format ! ("mysql://{username}:{password}@localhost?ssl_cert={ssl_cert}") ; let conn_opts = ConnectionOptions :: parse (db_url . as_str ()) . unwrap () ; let cstring = | s | CString :: new (s) . unwrap () ; assert_eq ! (Some (cstring ("localhost")) , conn_opts . host) ; assert_eq ! (None , conn_opts . port) ; assert_eq ! (cstring (username) , conn_opts . user) ; assert_eq ! (cstring (password) , conn_opts . password . unwrap ()) ; assert_eq ! (CString :: new (ssl_cert) . unwrap () , conn_opts . ssl_cert . unwrap ()) ; let url_with_unix_str_and_ssl_cert = format ! ("mysql://{username}:{password}@localhost?unix_socket=/var/run/mysqld.sock&ssl_cert={ssl_cert}") ; let conn_opts2 = ConnectionOptions :: parse (url_with_unix_str_and_ssl_cert . as_str ()) . unwrap () ; assert_eq ! (None , conn_opts2 . host) ; assert_eq ! (None , conn_opts2 . port) ; assert_eq ! (CString :: new (ssl_cert) . unwrap () , conn_opts2 . ssl_cert . unwrap ()) ; }
};
}
