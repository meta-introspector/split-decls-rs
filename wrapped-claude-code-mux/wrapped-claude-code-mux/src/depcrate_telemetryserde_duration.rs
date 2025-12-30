// Generated macro for serde_duration (module)
macro_rules! Depcrate_telemetryserde_duration {
() => {
// Module: crate::telemetry
// Provides: {"serde_duration"}
// Dependencies: {}
mod serde_duration { use serde :: { Serializer , Deserializer , Deserialize , de } ; use std :: time :: Duration ; pub fn serialize < S > (duration : & Duration , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . serialize_u64 (duration . as_millis () as u64) } pub fn deserialize < 'de , D > (deserializer : D) -> Result < Duration , D :: Error > where D : Deserializer < 'de > , { let millis = u64 :: deserialize (deserializer) ? ; Ok (Duration :: from_millis (millis)) } }
};
}
