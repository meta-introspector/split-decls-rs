// Generated macro for macro_557 (macro)
macro_rules! Depcrate_server_conn_automacro_557 {
() => {
// Module: crate::server::conn::auto
// Provides: {"macro_557"}
// Dependencies: {}
pin_project ! { # [project = ConnStateProj] enum ConnState <'a , I , S , E > where S : HttpService < Incoming >, { ReadVersion { # [pin] read_version : ReadVersion < I >, builder : Cow <'a , Builder < E >>, service : Option < S >, } , H1 { # [pin] conn : Http1Connection < I , S >, } , H2 { # [pin] conn : Http2Connection < I , S , E >, } , } }
};
}
