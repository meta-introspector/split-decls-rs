// Generated macro for macro_550 (macro)
macro_rules! Depcrate_server_conn_automacro_550 {
() => {
// Module: crate::server::conn::auto
// Provides: {"macro_550"}
// Dependencies: {}
pin_project ! { # [doc = " A [`Future`](core::future::Future) representing an HTTP/1 connection, returned from"] # [doc = " [`Builder::serve_connection`](struct.Builder.html#method.serve_connection)."] # [doc = ""] # [doc = " To drive HTTP on this connection this future **must be polled**, typically with"] # [doc = " `.await`. If it isn't polled, no progress will be made on this connection."] # [must_use = "futures do nothing unless polled"] pub struct Connection <'a , I , S , E > where S : HttpService < Incoming >, { # [pin] state : ConnState <'a , I , S , E >, } }
};
}
