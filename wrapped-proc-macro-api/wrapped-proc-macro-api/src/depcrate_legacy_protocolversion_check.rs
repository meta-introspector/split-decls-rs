// Generated macro for version_check (function)
macro_rules! Depcrate_legacy_protocolversion_check {
() => {
// Module: crate::legacy_protocol
// Provides: {"version_check"}
// Dependencies: {}
pub (crate) fn version_check (srv : & ProcMacroServerProcess) -> Result < u32 , ServerError > { let request = Request :: ApiVersionCheck { } ; let response = send_task (srv , request) ? ; match response { Response :: ApiVersionCheck (version) => Ok (version) , _ => Err (ServerError { message : "unexpected response" . to_owned () , io : None }) , } }
};
}
