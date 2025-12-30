// Generated macro for draw (function)
macro_rules! Depcratedraw {
() => {
// Module: crate
// Provides: {"draw"}
// Dependencies: {}
# [wasm_bindgen] pub fn draw (ctx : & CanvasRenderingContext2d , width : u32 , height : u32 , real : f64 , imaginary : f64 ,) -> Result < () , JsValue > { let c = Complex { real , imaginary } ; let data = get_julia_set (width , height , c) ; let data = ImageData :: new_with_u8_clamped_array_and_sh (Clamped (& data) , width , height) ? ; ctx . put_image_data (& data , 0.0 , 0.0) }
};
}
