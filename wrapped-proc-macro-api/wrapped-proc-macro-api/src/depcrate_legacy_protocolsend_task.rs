// Generated macro for send_task (function)
macro_rules! Depcrate_legacy_protocolsend_task {
() => {
// Module: crate::legacy_protocol
// Provides: {"send_task"}
// Dependencies: {}
# [doc = " Sends a request to the proc-macro server and waits for a response."] fn send_task (srv : & ProcMacroServerProcess , req : Request) -> Result < Response , ServerError > { if let Some (server_error) = srv . exited () { return Err (server_error . clone ()) ; } srv . send_task (send_request , req) }
};
}
