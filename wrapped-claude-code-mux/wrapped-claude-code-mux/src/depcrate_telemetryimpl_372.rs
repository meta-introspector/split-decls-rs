// Generated macro for impl_372 (impl)
macro_rules! Depcrate_telemetryimpl_372 {
() => {
// Module: crate::telemetry
// Provides: {"impl_372"}
// Dependencies: {}
impl RequestTelemetryExt for RequestBuilder { async fn track_telemetry < F , T > (& self , operation : F) -> Result < (T , RequestTelemetry) > where F : FnOnce () -> Result < T > + Send , T : Serialize + Send + 'static , { let start_time = Instant :: now () ; let request_id = uuid :: Uuid :: new_v4 () . to_string () ; let result = operation () ; let duration = start_time . elapsed () ; let telemetry = RequestTelemetry { request_id , start_time : SystemTime :: now () . duration_since (SystemTime :: UNIX_EPOCH) . expect ("Time went backwards") , duration , success : result . is_ok () , error_message : result . as_ref () . err () . map (| e | e . to_string ()) , request_size_bytes : 0 , response_size_bytes : 0 , } ; result . map (| data | (data , telemetry)) } fn record_metrics (& self , telemetry : RequestTelemetry) -> Result < () > { println ! ("Recording telemetry: {:?}" , telemetry) ; Ok (()) } fn get_telemetry_config (& self) -> TelemetryConfig { TelemetryConfig { enabled : true , upload_endpoint : "https://api.splitrail.dev/telemetry" . to_string () , batch_size : 100 , retry_attempts : 3 , } } }
};
}
