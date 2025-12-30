// Generated macro for HeadersThenBody (struct)
macro_rules! Depcrate_client_blocking_io_httpHeadersThenBody {
() => {
// Module: crate::client::blocking_io::http
// Provides: {"HeadersThenBody"}
// Dependencies: {}
struct HeadersThenBody < H : Http , B : Unpin > { service : Service , headers : Option < H :: Headers > , body : B , }
};
}
