// Generated macro for impl_1295 (impl)
macro_rules! Depcrate_style_textimpl_1295 {
() => {
// Module: crate::style::text
// Provides: {"impl_1295"}
// Dependencies: {}
impl < 'a > BackendTextStyle for TextStyle < 'a > { type FontError = FontError ; fn color (& self) -> BackendColor { self . color } fn size (& self) -> f64 { self . font . get_size () } fn transform (& self) -> FontTransform { self . font . get_transform () } fn style (& self) -> FontStyle { self . font . get_style () } # [allow (clippy :: type_complexity)] fn layout_box (& self , text : & str) -> Result < ((i32 , i32) , (i32 , i32)) , Self :: FontError > { self . font . layout_box (text) } fn anchor (& self) -> text_anchor :: Pos { self . pos } fn family (& self) -> FontFamily { self . font . get_family () } fn draw < E , DrawFunc : FnMut (i32 , i32 , BackendColor) -> Result < () , E > > (& self , text : & str , pos : BackendCoord , mut draw : DrawFunc ,) -> Result < Result < () , E > , Self :: FontError > { let color = self . color . color () ; self . font . draw (text , pos , move | x , y , a | { let mix_color = color . mix (a as f64) ; draw (x , y , mix_color) }) } }
};
}
