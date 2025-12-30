// Generated macro for bind_connection (function)
macro_rules! Depcrate_clientbind_connection {
() => {
// Module: crate::client
// Provides: {"bind_connection"}
// Dependencies: {}
async fn bind_connection < T > (io : & mut T) -> Result < () , crate :: Error > where T : AsyncRead + AsyncWrite + Unpin , { tracing :: debug ! ("binding client connection") ; let msg : & 'static [u8] = b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n" ; io . write_all (msg) . await . map_err (crate :: Error :: from_io) ? ; tracing :: debug ! ("client connection bound") ; Ok (()) }
};
}
