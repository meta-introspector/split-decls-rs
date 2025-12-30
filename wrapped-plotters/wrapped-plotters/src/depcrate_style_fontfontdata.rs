// Generated macro for FontData (trait)
macro_rules! Depcrate_style_fontFontData {
() => {
// Module: crate::style::font
// Provides: {"FontData"}
// Dependencies: {}
pub trait FontData : Clone { type ErrorType : Sized + std :: error :: Error + Clone ; fn new (family : FontFamily , style : FontStyle) -> Result < Self , Self :: ErrorType > ; fn estimate_layout (& self , size : f64 , text : & str) -> Result < LayoutBox , Self :: ErrorType > ; fn draw < E , DrawFunc : FnMut (i32 , i32 , f32) -> Result < () , E > > (& self , _pos : (i32 , i32) , _size : f64 , _text : & str , _draw : DrawFunc ,) -> Result < Result < () , E > , Self :: ErrorType > { panic ! ("The font implementation is unable to draw text") ; } }
};
}
