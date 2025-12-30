// Generated macro for ssl_mode (function)
macro_rules! Depcrate_mysql_connection_urlssl_mode {
() => {
// Module: crate::mysql::connection::url
// Provides: {"ssl_mode"}
// Dependencies: {}
# [test] fn ssl_mode () { let ssl_mode = | url | ConnectionOptions :: parse (url) . unwrap () . ssl_mode () ; assert_eq ! (ssl_mode ("mysql://localhost") , None) ; assert_eq ! (ssl_mode ("mysql://localhost?ssl_mode=disabled") , Some (mysql_ssl_mode :: SSL_MODE_DISABLED)) ; assert_eq ! (ssl_mode ("mysql://localhost?ssl_mode=PREFERRED") , Some (mysql_ssl_mode :: SSL_MODE_PREFERRED)) ; assert_eq ! (ssl_mode ("mysql://localhost?ssl_mode=required") , Some (mysql_ssl_mode :: SSL_MODE_REQUIRED)) ; assert_eq ! (ssl_mode ("mysql://localhost?ssl_mode=VERIFY_CA") , Some (mysql_ssl_mode :: SSL_MODE_VERIFY_CA)) ; assert_eq ! (ssl_mode ("mysql://localhost?ssl_mode=verify_identity") , Some (mysql_ssl_mode :: SSL_MODE_VERIFY_IDENTITY)) ; }
};
}
