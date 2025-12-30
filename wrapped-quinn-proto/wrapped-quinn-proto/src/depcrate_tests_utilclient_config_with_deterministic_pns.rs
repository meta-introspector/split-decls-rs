// Generated macro for client_config_with_deterministic_pns (function)
macro_rules! Depcrate_tests_utilclient_config_with_deterministic_pns {
() => {
// Module: crate::tests::util
// Provides: {"client_config_with_deterministic_pns"}
// Dependencies: {}
pub (super) fn client_config_with_deterministic_pns () -> ClientConfig { let mut cfg = ClientConfig :: new (Arc :: new (client_crypto ())) ; let mut transport = TransportConfig :: default () ; transport . deterministic_packet_numbers (true) ; cfg . transport = Arc :: new (transport) ; cfg }
};
}
