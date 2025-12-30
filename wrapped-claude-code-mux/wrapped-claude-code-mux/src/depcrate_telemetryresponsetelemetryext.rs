// Generated macro for ResponseTelemetryExt (trait)
macro_rules! Depcrate_telemetryResponseTelemetryExt {
() => {
// Module: crate::telemetry
// Provides: {"ResponseTelemetryExt"}
// Dependencies: {}
# [doc = " Trait for response telemetry and validation"] pub trait ResponseTelemetryExt { # [doc = " Parse response with telemetry tracking"] async fn parse_with_telemetry < T > (self) -> Result < (T , ResponseTelemetry) > where T : serde :: de :: DeserializeOwned + Send + 'static ; # [doc = " Validate response structure and collect metrics"] fn validate_response (& self , expected_size : Option < usize >) -> Result < ResponseValidation > ; }
};
}
