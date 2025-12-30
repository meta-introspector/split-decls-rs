// Generated macro for transport_config (function)
macro_rules! Depcratetransport_config {
() => {
// Module: crate
// Provides: {"transport_config"}
// Dependencies: {}
pub fn transport_config (opt : & Opt) -> quinn :: TransportConfig { let mut config = quinn :: TransportConfig :: default () ; config . max_concurrent_uni_streams (opt . max_streams . try_into () . unwrap ()) ; config . initial_mtu (opt . initial_mtu) ; let mut acks = quinn :: AckFrequencyConfig :: default () ; acks . ack_eliciting_threshold (10u32 . into ()) ; config . ack_frequency_config (Some (acks)) ; config }
};
}
