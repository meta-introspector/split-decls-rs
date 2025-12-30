// Generated macro for TextStyle (struct)
macro_rules! Depcrate_style_textTextStyle {
() => {
// Module: crate::style::text
// Provides: {"TextStyle"}
// Dependencies: {}
# [doc = " Style of a text"] # [derive (Clone)] pub struct TextStyle < 'a > { # [doc = " The font description"] pub font : FontDesc < 'a > , # [doc = " The text color"] pub color : BackendColor , # [doc = " The anchor point position"] pub pos : text_anchor :: Pos , }
};
}
