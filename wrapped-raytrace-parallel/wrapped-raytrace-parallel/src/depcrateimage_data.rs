// Generated macro for image_data (function)
macro_rules! Depcrateimage_data {
() => {
// Module: crate
// Provides: {"image_data"}
// Dependencies: {}
fn image_data (base : usize , len : usize , width : u32 , height : u32) -> ImageData { let mem = wasm_bindgen :: memory () . unchecked_into :: < WebAssembly :: Memory > () ; let mem = Uint8ClampedArray :: new (& mem . buffer ()) . slice (base as u32 , (base + len) as u32) ; ImageData :: new_with_js_u8_clamped_array_and_sh (& mem , width , height) . unwrap () }
};
}
