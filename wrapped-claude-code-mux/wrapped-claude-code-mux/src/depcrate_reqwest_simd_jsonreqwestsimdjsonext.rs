// Generated macro for ReqwestSimdJsonExt (trait)
macro_rules! Depcrate_reqwest_simd_jsonReqwestSimdJsonExt {
() => {
// Module: crate::reqwest_simd_json
// Provides: {"ReqwestSimdJsonExt"}
// Dependencies: {}
pub trait ReqwestSimdJsonExt { # [doc = " Set the request body as JSON using simd-json for serialization"] fn simd_json < T > (self , json : & T) -> RequestBuilder where T : Serialize + ? Sized ; }
};
}
