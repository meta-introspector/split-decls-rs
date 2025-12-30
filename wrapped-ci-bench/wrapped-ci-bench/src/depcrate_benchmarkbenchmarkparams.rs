// Generated macro for BenchmarkParams (struct)
macro_rules! Depcrate_benchmarkBenchmarkParams {
() => {
// Module: crate::benchmark
// Provides: {"BenchmarkParams"}
// Dependencies: {}
# [doc = " Parameters associated to a benchmark"] # [derive (Clone , Debug)] pub struct BenchmarkParams { # [doc = " Which `CryptoProvider` to test."] # [doc = ""] # [doc = " The choice of cipher suite is baked into this."] pub provider : Arc < CryptoProvider > , # [doc = " How to make a suitable [`rustls::crypto::TicketProducer`]."] pub ticketer : & 'static fn () -> Arc < dyn TicketProducer > , # [doc = " Where to get keys for server auth"] pub auth_key : AuthKeySource , # [doc = " TLS version"] pub version : ProtocolVersion , # [doc = " A user-facing label that identifies these params"] pub label : String , # [doc = " Call this once this BenchmarkParams is sure to be used"] pub warm_up : Option < fn () > , }
};
}
