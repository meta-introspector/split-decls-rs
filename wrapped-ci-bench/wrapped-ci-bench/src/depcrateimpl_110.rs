// Generated macro for impl_110 (impl)
macro_rules! Depcrateimpl_110 {
() => {
// Module: crate
// Provides: {"impl_110"}
// Dependencies: {}
# [async_trait (? Send)] impl BenchStepper for ServerSideStepper < '_ > { type Endpoint = ServerConnection ; async fn handshake (& mut self) -> anyhow :: Result < Self :: Endpoint > { let mut server = ServerConnection :: new (self . config . clone ()) . unwrap () ; server . set_buffer_limit (None) ; while server . is_handshaking () { read_handshake_message (& mut server , self . io . reader , self . io . handshake_buf) . await ? ; send_handshake_message (& mut server , self . io . writer , self . io . handshake_buf) . await ? ; } Ok (server) } async fn sync_before_resumed_handshake (& mut self) -> anyhow :: Result < () > { self . io . writer . write_all (& [42]) . await ? ; self . io . writer . flush () . await ? ; Ok (()) } async fn transmit_data (& mut self , endpoint : & mut Self :: Endpoint) -> anyhow :: Result < () > { write_all_plaintext_bounded (endpoint , self . io . writer , TRANSFER_PLAINTEXT_SIZE) . await ? ; Ok (()) } fn handshake_kind (& self , endpoint : & Self :: Endpoint) -> HandshakeKind { endpoint . handshake_kind () . unwrap () } }
};
}
