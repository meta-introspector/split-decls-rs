// Generated macro for impl_236 (impl)
macro_rules! Depcrate_reqwest_simd_jsonimpl_236 {
() => {
// Module: crate::reqwest_simd_json
// Provides: {"impl_236"}
// Dependencies: {}
impl ReqwestSimdJsonExt for RequestBuilder { fn simd_json < T > (self , json : & T) -> RequestBuilder where T : Serialize + ? Sized , { let body = simd_json :: to_vec (json) . expect ("Failed to serialize JSON") ; self . header (reqwest :: header :: CONTENT_TYPE , "application/json") . body (body) } }
};
}
