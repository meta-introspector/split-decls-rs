// Generated macro for ReqQueue (struct)
macro_rules! Depcrate_req_queueReqQueue {
() => {
// Module: crate::req_queue
// Provides: {"ReqQueue"}
// Dependencies: {}
# [doc = " Manages the set of pending requests, both incoming and outgoing."] # [derive (Debug)] pub struct ReqQueue < I , O > { pub incoming : Incoming < I > , pub outgoing : Outgoing < O > , }
};
}
