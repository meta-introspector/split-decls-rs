// Generated macro for RefLogMessage (enum)
macro_rules! Depcrate_remote_connection_fetchRefLogMessage {
() => {
// Module: crate::remote::connection::fetch
// Provides: {"RefLogMessage"}
// Dependencies: {}
# [doc = " The way reflog messages should be composed whenever a ref is written with recent objects from a remote."] pub enum RefLogMessage { # [doc = " Prefix the log with `action` and generate the typical suffix as `git` would."] Prefixed { # [doc = " The action to use, like `fetch` or `pull`."] action : String , } , # [doc = " Control the entire message, using `message` verbatim."] Override { # [doc = " The complete reflog message."] message : BString , } , }
};
}
