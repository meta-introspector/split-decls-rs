// Generated macro for connection_url_error (function)
macro_rules! Depcrate_mysql_connection_urlconnection_url_error {
() => {
// Module: crate::mysql::connection::url
// Provides: {"connection_url_error"}
// Dependencies: {}
fn connection_url_error () -> ConnectionError { let msg = "MySQL connection URLs must be in the form \
               `mysql://[[user]:[password]@]host[:port][/database][?unix_socket=socket-path]`" ; ConnectionError :: InvalidConnectionUrl (msg . into ()) }
};
}
