// Generated macro for Error (struct)
macro_rules! Depcrate_client_legacy_clientError {
() => {
// Module: crate::client::legacy::client
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Client errors"] pub struct Error { kind : ErrorKind , source : Option < Box < dyn StdError + Send + Sync > > , # [cfg (any (feature = "http1" , feature = "http2"))] connect_info : Option < Connected > , }
};
}
