// Generated macro for BenchStepper (trait)
macro_rules! DepcrateBenchStepper {
() => {
// Module: crate
// Provides: {"BenchStepper"}
// Dependencies: {}
# [doc = " Drives the different steps in a benchmark."] # [doc = ""] # [doc = " See [`run_bench`] for specific details on how it is used."] # [async_trait (? Send)] trait BenchStepper { type Endpoint ; async fn handshake (& mut self) -> anyhow :: Result < Self :: Endpoint > ; async fn sync_before_resumed_handshake (& mut self) -> anyhow :: Result < () > ; async fn transmit_data (& mut self , endpoint : & mut Self :: Endpoint) -> anyhow :: Result < () > ; fn handshake_kind (& self , endpoint : & Self :: Endpoint) -> HandshakeKind ; }
};
}
