// Generated macro for impl_12 (impl)
macro_rules! Depcrate_codecimpl_12 {
() => {
// Module: crate::codec
// Provides: {"impl_12"}
// Dependencies: {}
impl Codec for TransferrableCodec { fn encode < I > (input : I) -> wasm_bindgen :: JsValue where I : serde :: Serialize , { serde_wasm_bindgen :: to_value (& input) . expect ("failed to encode") } fn decode < O > (input : wasm_bindgen :: JsValue) -> O where O : for < 'de > serde :: Deserialize < 'de > , { serde_wasm_bindgen :: from_value (input) . expect ("failed to decode") } }
};
}
