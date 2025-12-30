// Generated macro for render_line (function)
macro_rules! Depcrate_paragraphrender_line {
() => {
// Module: crate::paragraph
// Provides: {"render_line"}
// Dependencies: {}
fn render_line (wrapped : & WrappedLine < '_ , '_ > , area : Rect , buf : & mut Buffer , y : u16) { let mut x = get_line_offset (wrapped . width , area . width , wrapped . alignment) ; for StyledGrapheme { symbol , style } in wrapped . graphemes { let width = symbol . width () ; if width == 0 { continue ; } let symbol = if symbol . is_empty () { " " } else { symbol } ; let position = Position :: new (area . left () + x , area . top () + y) ; buf [position] . set_symbol (symbol) . set_style (* style) ; x += u16 :: try_from (width) . unwrap_or (u16 :: MAX) ; } }
};
}
