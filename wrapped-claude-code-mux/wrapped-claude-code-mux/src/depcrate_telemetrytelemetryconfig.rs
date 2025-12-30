// Generated macro for TelemetryConfig (struct)
macro_rules! Depcrate_telemetryTelemetryConfig {
() => {
// Module: crate::telemetry
// Provides: {"TelemetryConfig"}
// Dependencies: {}
# [doc = " Configuration for telemetry collection"] # [derive (Debug , Clone)] pub struct TelemetryConfig { pub enabled : bool , pub upload_endpoint : String , pub batch_size : usize , pub retry_attempts : u32 , }
};
}
