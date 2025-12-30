// Generated macro for impl_1200 (impl)
macro_rules! Depcrate_style_font_naiveimpl_1200 {
() => {
// Module: crate::style::font::naive
// Provides: {"impl_1200"}
// Dependencies: {}
impl FontData for FontDataInternal { type ErrorType = FontError ; fn new (family : FontFamily , style : FontStyle) -> Result < Self , FontError > { Ok (FontDataInternal (family . as_str () . into () , style . as_str () . into () ,)) } # [doc = " Note: This is only a crude estimatation, since for some backend such as SVG, we have no way to"] # [doc = " know the real size of the text anyway. Thus using font-kit is an overkill and doesn't helps"] # [doc = " the layout."] fn estimate_layout (& self , size : f64 , text : & str) -> Result < LayoutBox , Self :: ErrorType > { let em = size / 1.24 / 1.24 ; Ok (((0 , - em . round () as i32) , ((em * 0.7 * text . len () as f64) . round () as i32 , (em * 0.24) . round () as i32 ,) ,)) } }
};
}
