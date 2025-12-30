// Generated macro for MethodReply (struct)
macro_rules! Depcrate_nonblockMethodReply {
() => {
// Module: crate::nonblock
// Provides: {"MethodReply"}
// Dependencies: {}
# [doc = " Future method reply, used while waiting for a method call reply from the server."] pub struct MethodReply < T > (pin :: Pin < Box < dyn Future < Output = Result < T , Error > > + Send + 'static > >) ;
};
}
