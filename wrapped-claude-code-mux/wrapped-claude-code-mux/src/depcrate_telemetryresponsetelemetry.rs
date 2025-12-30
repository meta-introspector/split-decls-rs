// Generated macro for ResponseTelemetry (struct)
macro_rules! Depcrate_telemetryResponseTelemetry {
() => {
// Module: crate::telemetry
// Provides: {"ResponseTelemetry"}
// Dependencies: {}
# [doc = " Response telemetry data"] # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ResponseTelemetry { # [serde (with = "crate::telemetry::serde_duration")] pub parse_duration : Duration , pub parse_success : bool , pub response_size : usize , pub content_type : Option < String > , pub status_code : u16 , }
};
}
