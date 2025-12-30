// Generated macro for impl_43 (impl)
macro_rules! Depcrate_benchmarkimpl_43 {
() => {
// Module: crate::benchmark
// Provides: {"impl_43"}
// Dependencies: {}
impl BenchmarkParams { # [doc = " Create a new set of benchmark params"] pub const fn new (provider : Arc < CryptoProvider > , ticketer : & 'static fn () -> Arc < dyn TicketProducer > , auth_key : AuthKeySource , version : ProtocolVersion , label : String , warm_up : Option < fn () > ,) -> Self { Self { provider , ticketer , auth_key , version , label , warm_up , } } }
};
}
