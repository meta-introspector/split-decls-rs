// Generated macro for impl_59 (impl)
macro_rules! Depcrate_serverimpl_59 {
() => {
// Module: crate::server
// Provides: {"impl_59"}
// Dependencies: {}
impl Stream for ServerEventMultiplexer { type Item = ServerCommand ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let this = Pin :: into_inner (self) ; if let Some (signal_fut) = & mut this . signal_fut { if let Poll :: Ready (signal) = Pin :: new (signal_fut) . poll (cx) { this . signal_fut = None ; return Poll :: Ready (Some (ServerInner :: map_signal (signal))) ; } } this . cmd_rx . poll_recv (cx) } }
};
}
