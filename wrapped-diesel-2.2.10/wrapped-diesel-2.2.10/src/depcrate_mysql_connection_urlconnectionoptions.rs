// Generated macro for ConnectionOptions (struct)
macro_rules! Depcrate_mysql_connection_urlConnectionOptions {
() => {
// Module: crate::mysql::connection::url
// Provides: {"ConnectionOptions"}
// Dependencies: {}
pub (super) struct ConnectionOptions { host : Option < CString > , user : CString , password : Option < CString > , database : Option < CString > , port : Option < u16 > , unix_socket : Option < CString > , client_flags : CapabilityFlags , ssl_mode : Option < mysql_ssl_mode > , ssl_ca : Option < CString > , ssl_cert : Option < CString > , ssl_key : Option < CString > , }
};
}
