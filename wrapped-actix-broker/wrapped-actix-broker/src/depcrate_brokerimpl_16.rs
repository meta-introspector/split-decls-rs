// Generated macro for impl_16 (impl)
macro_rules! Depcrate_brokerimpl_16 {
() => {
// Module: crate::broker
// Provides: {"impl_16"}
// Dependencies: {}
impl < T : 'static + Unpin , M : BrokerMsg > Handler < IssueAsync < M > > for Broker < T > { type Result = () ; fn handle (& mut self , msg : IssueAsync < M > , _ctx : & mut Context < Self >) { trace ! ("Broker: Received IssueAsync") ; let subscriber_indexes = if let Some (subscribers) = self . get_subs :: < M > () { subscribers . filter (| & (_ , (actor_id , _)) | ! msg . 1 . eq (actor_id)) . filter_map (| (idx , (_ , recipient)) | match recipient . try_send (msg . 0 . clone ()) { Err (SendError :: Full (msg)) => { recipient . do_send (msg) ; None } Err (_) => Some (idx) , _ => None , } ,) . collect () } else { vec ! [] } ; self . remove_subs :: < M > (subscriber_indexes . as_slice ()) ; self . set_msg :: < M > (msg . 0) ; } }
};
}
