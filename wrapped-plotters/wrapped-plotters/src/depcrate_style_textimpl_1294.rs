// Generated macro for impl_1294 (impl)
macro_rules! Depcrate_style_textimpl_1294 {
() => {
// Module: crate::style::text
// Provides: {"impl_1294"}
// Dependencies: {}
impl < 'a , T : Into < FontDesc < 'a > > > From < T > for TextStyle < 'a > { fn from (font : T) -> Self { Self { font : font . into () , color : BLACK . to_backend_color () , pos : text_anchor :: Pos :: default () , } } }
};
}
