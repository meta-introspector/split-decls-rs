// Generated macro for Response (struct)
macro_rules! Depcrate_fetch_typesResponse {
() => {
// Module: crate::fetch::types
// Provides: {"Response"}
// Dependencies: {}
# [doc = " A representation of a complete fetch response"] # [derive (Debug , Clone)] pub struct Response { pub (crate) acks : Vec < Acknowledgement > , pub (crate) shallows : Vec < ShallowUpdate > , pub (crate) wanted_refs : Vec < WantedRef > , pub (crate) has_pack : bool , }
};
}
