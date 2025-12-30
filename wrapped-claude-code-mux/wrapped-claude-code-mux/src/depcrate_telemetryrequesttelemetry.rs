// Generated macro for RequestTelemetry (struct)
macro_rules! Depcrate_telemetryRequestTelemetry {
() => {
// Module: crate::telemetry
// Provides: {"RequestTelemetry"}
// Dependencies: {}
# [doc = " Core telemetry data for request handling"] # [derive (Debug , Clone , Serialize , Deserialize)] pub struct RequestTelemetry { pub request_id : String , # [serde (with = "crate::telemetry::serde_duration")] pub start_time : Duration , # [serde (with = "crate::telemetry::serde_duration")] pub duration : Duration , pub success : bool , pub error_message : Option < String > , pub request_size_bytes : usize , pub response_size_bytes : usize , }
};
}
