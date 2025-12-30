// Generated macro for impl_373 (impl)
macro_rules! Depcrate_telemetryimpl_373 {
() => {
// Module: crate::telemetry
// Provides: {"impl_373"}
// Dependencies: {}
impl ResponseTelemetryExt for Response { async fn parse_with_telemetry < T > (self) -> Result < (T , ResponseTelemetry) > where T : serde :: de :: DeserializeOwned + Send + 'static , { let start_time = Instant :: now () ; let size = self . content_length () . unwrap_or (0) as usize ; let content_type = self . headers () . get (reqwest :: header :: CONTENT_TYPE) . and_then (| v | v . to_str () . ok ()) . map (| s | s . to_string ()) ; let status_code = self . status () . as_u16 () ; let result = self . simd_json :: < T > () . await ; let duration = start_time . elapsed () ; let telemetry = ResponseTelemetry { parse_duration : duration , parse_success : result . is_ok () , response_size : size , content_type , status_code , } ; result . map (| data | (data , telemetry)) } fn validate_response (& self , expected_size : Option < usize >) -> Result < ResponseValidation > { let actual_size = self . content_length () . unwrap_or (0) as usize ; let size_matches = expected_size . map (| expected | actual_size == expected) . unwrap_or (true) ; let validation = ResponseValidation { is_valid : self . status () . is_success () && size_matches , size_matches , parse_success : true , validation_errors : if ! self . status () . is_success () { vec ! [format ! ("HTTP error: {}" , self . status ())] } else if ! size_matches { vec ! [format ! ("Size mismatch: expected {:?}, got {}" , expected_size , actual_size)] } else { vec ! [] } , } ; Ok (validation) } }
};
}
