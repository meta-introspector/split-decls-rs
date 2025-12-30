// Generated macro for ConnectMode (enum)
macro_rules! Depcrate_client_gitConnectMode {
() => {
// Module: crate::client::git
// Provides: {"ConnectMode"}
// Dependencies: {}
# [doc = " The way to connect to a process speaking the `git` protocol."] # [derive (PartialEq , Eq , Clone , Copy)] pub enum ConnectMode { # [doc = " A git daemon."] Daemon , # [doc = " A spawned `git` process to upload a pack to the client."] Process , }
};
}
