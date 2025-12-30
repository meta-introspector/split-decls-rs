// Generated macro for impl_890 (impl)
macro_rules! Depcrate_shareimpl_890 {
() => {
// Module: crate::share
// Provides: {"impl_890"}
// Dependencies: {}
# [cfg (feature = "stream")] impl futures_core :: Stream for RecvStream { type Item = Result < Bytes , crate :: Error > ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { self . poll_data (cx) } }
};
}
