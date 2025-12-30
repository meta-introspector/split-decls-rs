// Generated macro for impl_501 (impl)
macro_rules! Depcrate_webimpl_501 {
() => {
// Module: crate::web
// Provides: {"impl_501"}
// Dependencies: {}
impl < T , U > Into < JsValue > for ChartInfo < T , U > where T : serde :: Serialize , U : serde :: Serialize , { fn into (self) -> JsValue { serde_wasm_bindgen :: to_value (& (self . chart_bounds , self . plot_ranges)) . unwrap () } }
};
}
