// Generated macro for SendMsg (type)
macro_rules! Depcrate_future_future_remote_handleSendMsg {
() => {
// Module: crate::future::future::remote_handle
// Provides: {"SendMsg"}
// Dependencies: {}
type SendMsg < Fut > = Result < < Fut as Future > :: Output , Box < dyn Any + Send + 'static > > ;
};
}
