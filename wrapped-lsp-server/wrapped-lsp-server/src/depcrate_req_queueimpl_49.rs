// Generated macro for impl_49 (impl)
macro_rules! Depcrate_req_queueimpl_49 {
() => {
// Module: crate::req_queue
// Provides: {"impl_49"}
// Dependencies: {}
impl < O > Outgoing < O > { pub fn register < P : serde :: Serialize > (& mut self , method : String , params : P , data : O) -> Request { let id = RequestId :: from (self . next_id) ; self . pending . insert (id . clone () , data) ; self . next_id += 1 ; Request :: new (id , method , params) } pub fn complete (& mut self , id : RequestId) -> Option < O > { self . pending . remove (& id) } }
};
}
