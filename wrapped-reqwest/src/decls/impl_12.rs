macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        # [cfg (target_arch = "wasm32")] impl From < crate :: error :: Error > for wasm_bindgen :: JsValue { fn from (err : Error) -> wasm_bindgen :: JsValue { js_sys :: Error :: from (err) . into () } }
    };
}

impl_12!()