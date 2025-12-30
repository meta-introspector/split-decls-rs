// Generated macro for RequestTelemetryExt (trait)
macro_rules! Depcrate_telemetryRequestTelemetryExt {
() => {
// Module: crate::telemetry
// Provides: {"RequestTelemetryExt"}
// Dependencies: {}
# [doc = " Trait for adding telemetry capabilities to request handlers"] pub trait RequestTelemetryExt { # [doc = " Track request execution with automatic timing and error capture"] async fn track_telemetry < F , T > (& self , operation : F) -> Result < (T , RequestTelemetry) > where F : FnOnce () -> Result < T > + Send , T : Serialize + Send + 'static ; # [doc = " Record request metrics for upload to Splitrail Cloud"] fn record_metrics (& self , telemetry : RequestTelemetry) -> Result < () > ; # [doc = " Get telemetry configuration settings"] fn get_telemetry_config (& self) -> TelemetryConfig ; }
};
}
