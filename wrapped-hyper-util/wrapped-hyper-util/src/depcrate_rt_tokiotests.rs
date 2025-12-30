// Generated macro for tests (module)
macro_rules! Depcrate_rt_tokiotests {
() => {
// Module: crate::rt::tokio
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: rt :: TokioExecutor ; use hyper :: rt :: Executor ; use tokio :: sync :: oneshot ; # [tokio :: test] async fn simple_execute () -> Result < () , Box < dyn std :: error :: Error > > { let (tx , rx) = oneshot :: channel () ; let executor = TokioExecutor :: new () ; executor . execute (async move { tx . send (()) . unwrap () ; }) ; rx . await . map_err (Into :: into) } }
};
}
