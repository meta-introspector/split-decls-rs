macro_rules! deps {
    () => {
        BoxError!();
    };
}

macro_rules! macro_23 {
    () => {
        deps!();
        if_wasm ! { pub (crate) fn wasm (js_val : wasm_bindgen :: JsValue) -> BoxError { format ! ("{js_val:?}") . into () } }
    };
}

macro_23!()