// Generated macro for macro_31 (macro)
macro_rules! Depcrate_client_async_io_requestmacro_31 {
() => {
// Module: crate::client::async_io::request
// Provides: {"macro_31"}
// Dependencies: {}
pin_project ! { # [doc = " A [`Write`][io::Write] implementation optimized for writing packet lines."] # [doc = " A type implementing `Write` for packet lines, which when done can be transformed into a `Read` for"] # [doc = " obtaining the response."] pub struct RequestWriter <'a > { on_into_read : MessageKind , # [pin] writer : Writer < Box < dyn AsyncWrite + Unpin + 'a >>, reader : Box < dyn ExtendedBufRead <'a > + Unpin + 'a >, trace : bool , } }
};
}
