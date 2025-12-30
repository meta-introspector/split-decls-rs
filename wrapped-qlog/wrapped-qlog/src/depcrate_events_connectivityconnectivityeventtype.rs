// Generated macro for ConnectivityEventType (enum)
macro_rules! Depcrate_events_connectivityConnectivityEventType {
() => {
// Module: crate::events::connectivity
// Provides: {"ConnectivityEventType"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Clone , Copy , PartialEq , Eq , Debug)] # [serde (rename_all = "snake_case")] pub enum ConnectivityEventType { ServerListening , ConnectionStarted , ConnectionClosed , ConnectionIdUpdated , SpinBitUpdated , ConnectionStateUpdated , MtuUpdated , }
};
}
