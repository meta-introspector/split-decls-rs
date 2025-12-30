// Generated macro for impl_286 (impl)
macro_rules! Depcrate_proto_connectionimpl_286 {
() => {
// Module: crate::proto::connection
// Provides: {"impl_286"}
// Dependencies: {}
impl < P , B > ConnectionInner < P , B > where P : Peer , B : Buf , { fn as_dyn (& mut self) -> DynConnection < '_ , B > { let ConnectionInner { state , go_away , streams , error , ping_pong , .. } = self ; let streams = streams . as_dyn () ; DynConnection { state , go_away , streams , error , ping_pong , } } }
};
}
