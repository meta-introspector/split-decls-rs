// Generated macro for conversion (module)
macro_rules! Depcrate_errorconversion {
() => {
// Module: crate::error
// Provides: {"conversion"}
// Dependencies: {}
# [cfg (any (feature = "http" , feature = "websocket" , feature = "eventsource"))] mod conversion { use gloo_utils :: errors :: JsError ; use std :: convert :: TryFrom ; use wasm_bindgen :: JsValue ; # [cfg (feature = "http")] pub (crate) fn js_to_error (js_value : JsValue) -> super :: Error { super :: Error :: JsError (js_to_js_error (js_value)) } pub (crate) fn js_to_js_error (js_value : JsValue) -> JsError { match JsError :: try_from (js_value) { Ok (error) => error , Err (_) => unreachable ! ("JsValue passed is not an Error type -- this is a bug") , } } }
};
}
