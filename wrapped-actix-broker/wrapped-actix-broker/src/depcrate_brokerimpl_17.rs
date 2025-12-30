// Generated macro for impl_17 (impl)
macro_rules! Depcrate_brokerimpl_17 {
() => {
// Module: crate::broker
// Provides: {"impl_17"}
// Dependencies: {}
impl < T : 'static + Unpin , M : BrokerMsg > Handler < IssueSync < M > > for Broker < T > { type Result = () ; fn handle (& mut self , msg : IssueSync < M > , ctx : & mut Context < Self >) { trace ! ("Broker: Received IssueSync") ; if let Some (subscribers) = self . get_subs :: < M > () { subscribers . filter (| & (_ , (actor_id , _)) | ! msg . 1 . eq (actor_id)) . for_each (| (_ , (_ , recipient)) | { recipient . send (msg . 0 . clone ()) . into_actor (self) . map (| _ , _ , _ | ()) . wait (ctx) ; }) ; } self . set_msg :: < M > (msg . 0) ; } }
};
}
