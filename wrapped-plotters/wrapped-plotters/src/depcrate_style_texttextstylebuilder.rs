// Generated macro for TextStyleBuilder (struct)
macro_rules! Depcrate_style_textTextStyleBuilder {
() => {
// Module: crate::style::text
// Provides: {"TextStyleBuilder"}
// Dependencies: {}
pub struct TextStyleBuilder < 'a , T : IntoTextStyle < 'a > > { base : T , new_color : Option < BackendColor > , new_pos : Option < text_anchor :: Pos > , _phatom : std :: marker :: PhantomData < & 'a T > , }
};
}
