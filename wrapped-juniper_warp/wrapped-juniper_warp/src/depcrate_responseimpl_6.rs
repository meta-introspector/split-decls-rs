// Generated macro for impl_6 (impl)
macro_rules! Depcrate_responseimpl_6 {
() => {
// Module: crate::response
// Provides: {"impl_6"}
// Dependencies: {}
impl < S > Reply for JuniperResponse < S > where S : ScalarValue + Send , { fn into_response (self) -> reply :: Response { match serde_json :: to_vec (& self . 0) { Ok (json) => http :: Response :: builder () . status (if self . 0 . is_ok () { StatusCode :: OK } else { StatusCode :: BAD_REQUEST }) . header ("content-type" , "application/json") . body (json . into ()) , Err (e) => http :: Response :: builder () . status (StatusCode :: INTERNAL_SERVER_ERROR) . body (e . to_string () . into ()) , } . unwrap_or_else (| e | { unreachable ! ("cannot build `reply::Response` out of `JuniperResponse`: {e}") }) } }
};
}
