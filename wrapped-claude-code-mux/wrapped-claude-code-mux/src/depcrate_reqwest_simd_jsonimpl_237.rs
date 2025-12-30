// Generated macro for impl_237 (impl)
macro_rules! Depcrate_reqwest_simd_jsonimpl_237 {
() => {
// Module: crate::reqwest_simd_json
// Provides: {"impl_237"}
// Dependencies: {}
impl ResponseSimdJsonExt for Response { async fn simd_json < T > (self) -> Result < T > where T : serde :: de :: DeserializeOwned , { let bytes = self . bytes () . await ? ; let mut bytes = bytes . to_vec () ; let result = simd_json :: from_slice (& mut bytes) ? ; Ok (result) } }
};
}
