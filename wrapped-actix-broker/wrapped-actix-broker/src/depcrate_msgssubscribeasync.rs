// Generated macro for SubscribeAsync (struct)
macro_rules! Depcrate_msgsSubscribeAsync {
() => {
// Module: crate::msgs
// Provides: {"SubscribeAsync"}
// Dependencies: {}
# [derive (Message)] # [rtype (result = "()")] pub struct SubscribeAsync < M : BrokerMsg > (pub Recipient < M > , pub TypeId) ;
};
}
