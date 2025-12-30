// Generated macro for RequestWriter (struct)
macro_rules! Depcrate_client_blocking_io_requestRequestWriter {
() => {
// Module: crate::client::blocking_io::request
// Provides: {"RequestWriter"}
// Dependencies: {}
# [doc = " A [`Write`][io::Write] implementation optimized for writing packet lines."] # [doc = " A type implementing `Write` for packet lines, which when done can be transformed into a `Read` for"] # [doc = " obtaining the response."] pub struct RequestWriter < 'a > { on_into_read : MessageKind , writer : Writer < Box < dyn io :: Write + 'a > > , reader : Box < dyn ExtendedBufRead < 'a > + Unpin + 'a > , trace : bool , }
};
}
