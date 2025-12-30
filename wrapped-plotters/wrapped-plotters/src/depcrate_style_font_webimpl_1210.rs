// Generated macro for impl_1210 (impl)
macro_rules! Depcrate_style_font_webimpl_1210 {
() => {
// Module: crate::style::font::web
// Provides: {"impl_1210"}
// Dependencies: {}
impl FontData for FontDataInternal { type ErrorType = FontError ; fn new (family : FontFamily , style : FontStyle) -> Result < Self , FontError > { Ok (FontDataInternal (family . as_str () . into () , style . as_str () . into () ,)) } fn estimate_layout (& self , size : f64 , text : & str) -> Result < LayoutBox , Self :: ErrorType > { let window = window () . unwrap () ; let document = window . document () . unwrap () ; let body = document . body () . unwrap () ; let span = document . create_element ("span") . unwrap () ; span . set_text_content (Some (text)) ; span . set_attribute ("style" , & format ! ("display: inline-block; font-family:{}; font-style:{}; font-size: {}px; position: fixed; top: 100%" , self . 0 , self . 1 , size)) . unwrap () ; let span = span . into () ; body . append_with_node_1 (& span) . unwrap () ; let elem = JsCast :: dyn_into :: < HtmlElement > (span) . unwrap () ; let height = elem . offset_height () as i32 ; let width = elem . offset_width () as i32 ; elem . remove () ; Ok (((0 , 0) , (width , height))) } }
};
}
