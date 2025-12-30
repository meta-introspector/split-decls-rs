// Generated macro for ResponseSimdJsonExt (trait)
macro_rules! Depcrate_reqwest_simd_jsonResponseSimdJsonExt {
() => {
// Module: crate::reqwest_simd_json
// Provides: {"ResponseSimdJsonExt"}
// Dependencies: {}
pub trait ResponseSimdJsonExt { # [doc = " Parse response body as JSON using simd-json"] async fn simd_json < T > (self) -> Result < T > where T : serde :: de :: DeserializeOwned ; }
};
}
