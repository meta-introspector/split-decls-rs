// Generated macro for macro_560 (macro)
macro_rules! Depcrate_server_conn_automacro_560 {
() => {
// Module: crate::server::conn::auto
// Provides: {"macro_560"}
// Dependencies: {}
pin_project ! { # [doc = " An upgradable [`Connection`], returned by"] # [doc = " [`Builder::serve_upgradable_connection`](struct.Builder.html#method.serve_connection_with_upgrades)."] # [doc = ""] # [doc = " To drive HTTP on this connection this future **must be polled**, typically with"] # [doc = " `.await`. If it isn't polled, no progress will be made on this connection."] # [must_use = "futures do nothing unless polled"] pub struct UpgradeableConnection <'a , I , S , E > where S : HttpService < Incoming >, { # [pin] state : UpgradeableConnState <'a , I , S , E >, } }
};
}
