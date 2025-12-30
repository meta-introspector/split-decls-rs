// Generated macro for impl_107 (impl)
macro_rules! Depcrateimpl_107 {
() => {
// Module: crate
// Provides: {"impl_107"}
// Dependencies: {}
# [async_trait (? Send)] impl BenchStepper for ClientSideStepper < '_ > { type Endpoint = ClientConnection ; async fn handshake (& mut self) -> anyhow :: Result < Self :: Endpoint > { let server_name = "localhost" . try_into () . unwrap () ; let mut client = ClientConnection :: new (self . config . clone () , server_name) . unwrap () ; client . set_buffer_limit (None) ; loop { send_handshake_message (& mut client , self . io . writer , self . io . handshake_buf) . await ? ; if ! client . is_handshaking () && ! client . wants_write () { break ; } read_handshake_message (& mut client , self . io . reader , self . io . handshake_buf) . await ? ; } if self . resumption_kind != ResumptionKind :: No && client . protocol_version () . unwrap () == ProtocolVersion :: TLSv1_3 { read_handshake_message (& mut client , self . io . reader , self . io . handshake_buf) . await ? ; } Ok (client) } async fn sync_before_resumed_handshake (& mut self) -> anyhow :: Result < () > { let buf = & mut [0] ; self . io . reader . read_exact (buf) . await ? ; assert_eq ! (buf [0] , 42) ; Ok (()) } async fn transmit_data (& mut self , endpoint : & mut Self :: Endpoint) -> anyhow :: Result < () > { let total_plaintext_read = read_plaintext_to_end_bounded (endpoint , self . io . reader) . await ? ; assert_eq ! (total_plaintext_read , TRANSFER_PLAINTEXT_SIZE) ; Ok (()) } fn handshake_kind (& self , endpoint : & Self :: Endpoint) -> HandshakeKind { endpoint . handshake_kind () . unwrap () } }
};
}
