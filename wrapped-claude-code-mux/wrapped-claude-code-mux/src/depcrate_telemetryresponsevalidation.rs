// Generated macro for ResponseValidation (struct)
macro_rules! Depcrate_telemetryResponseValidation {
() => {
// Module: crate::telemetry
// Provides: {"ResponseValidation"}
// Dependencies: {}
# [doc = " Response validation metrics"] # [derive (Debug , Clone)] pub struct ResponseValidation { pub is_valid : bool , pub size_matches : bool , pub parse_success : bool , pub validation_errors : Vec < String > , }
};
}
