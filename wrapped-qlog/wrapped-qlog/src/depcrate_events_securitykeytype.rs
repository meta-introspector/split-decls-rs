// Generated macro for KeyType (enum)
macro_rules! Depcrate_events_securityKeyType {
() => {
// Module: crate::events::security
// Provides: {"KeyType"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug , Default)] # [serde (rename_all = "snake_case")] pub enum KeyType { ServerInitialSecret , ClientInitialSecret , ServerHandshakeSecret , ClientHandshakeSecret , # [serde (rename = "server_0rtt_secret")] Server0RttSecret , # [serde (rename = "client_0rtt_secret")] Client0RttSecret , # [serde (rename = "server_1rtt_secret")] Server1RttSecret , # [serde (rename = "client_1rtt_secret")] Client1RttSecret , # [default] Unknown , }
};
}
