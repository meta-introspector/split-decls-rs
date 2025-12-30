// Generated macro for impl_24 (impl)
macro_rules! Depcrateimpl_24 {
() => {
// Module: crate
// Provides: {"impl_24"}
// Dependencies: {}
# [wasm_bindgen] impl RenderingScene { # [doc = " Returns the JS promise object which resolves when the render is complete"] pub fn promise (& self) -> Promise { self . promise . clone () } # [doc = " Return a progressive rendering of the image so far"] # [wasm_bindgen (js_name = imageSoFar)] pub fn image_so_far (& self) -> ImageData { image_data (self . base , self . len , self . width , self . height) } }
};
}
