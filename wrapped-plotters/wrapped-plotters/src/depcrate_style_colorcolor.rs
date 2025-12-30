// Generated macro for Color (trait)
macro_rules! Depcrate_style_colorColor {
() => {
// Module: crate::style::color
// Provides: {"Color"}
// Dependencies: {}
# [doc = " Any color representation"] pub trait Color { # [doc = " Normalize this color representation to the backend color"] fn to_backend_color (& self) -> BackendColor ; # [doc = " Convert the RGB representation to the standard RGB tuple"] # [inline (always)] fn rgb (& self) -> (u8 , u8 , u8) { self . to_backend_color () . rgb } # [doc = " Get the alpha channel of the color"] # [inline (always)] fn alpha (& self) -> f64 { self . to_backend_color () . alpha } # [doc = " Mix the color with given opacity"] fn mix (& self , value : f64) -> RGBAColor { let (r , g , b) = self . rgb () ; let a = self . alpha () * value ; RGBAColor (r , g , b , a) } # [doc = " Convert the color into the RGBA color which is internally used by Plotters"] fn to_rgba (& self) -> RGBAColor { let (r , g , b) = self . rgb () ; let a = self . alpha () ; RGBAColor (r , g , b , a) } # [doc = " Make a filled style form the color"] fn filled (& self) -> ShapeStyle where Self : Sized , { Into :: < ShapeStyle > :: into (self) . filled () } # [doc = " Make a shape style with stroke width from a color"] fn stroke_width (& self , width : u32) -> ShapeStyle where Self : Sized , { Into :: < ShapeStyle > :: into (self) . stroke_width (width) } }
};
}
