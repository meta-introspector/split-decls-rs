// Generated macro for macro_563 (macro)
macro_rules! Depcrate_server_conn_automacro_563 {
() => {
// Module: crate::server::conn::auto
// Provides: {"macro_563"}
// Dependencies: {}
pin_project ! { # [project = UpgradeableConnStateProj] enum UpgradeableConnState <'a , I , S , E > where S : HttpService < Incoming >, { ReadVersion { # [pin] read_version : ReadVersion < I >, builder : Cow <'a , Builder < E >>, service : Option < S >, } , H1 { # [pin] conn : Http1UpgradeableConnection < Rewind < I >, S >, } , H2 { # [pin] conn : Http2Connection < I , S , E >, } , } }
};
}
