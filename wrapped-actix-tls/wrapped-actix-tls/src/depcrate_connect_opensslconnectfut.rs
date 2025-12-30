// Generated macro for ConnectFut (struct)
macro_rules! Depcrate_connect_opensslConnectFut {
() => {
// Module: crate::connect::openssl
// Provides: {"ConnectFut"}
// Dependencies: {}
# [doc = " Connect future for OpenSSL service."] # [doc (hidden)] pub struct ConnectFut < R , IO > { io : Option < AsyncSslStream < IO > > , stream : Option < Connection < R , () > > , }
};
}
