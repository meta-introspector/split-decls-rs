// Generated macro for impl_45 (impl)
macro_rules! Depcrate_req_queueimpl_45 {
() => {
// Module: crate::req_queue
// Provides: {"impl_45"}
// Dependencies: {}
impl < I , O > Default for ReqQueue < I , O > { fn default () -> ReqQueue < I , O > { ReqQueue { incoming : Incoming { pending : HashMap :: default () } , outgoing : Outgoing { next_id : 0 , pending : HashMap :: default () } , } } }
};
}
