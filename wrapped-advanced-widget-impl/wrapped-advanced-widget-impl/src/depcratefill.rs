// Generated macro for fill (function)
macro_rules! Depcratefill {
() => {
// Module: crate
// Provides: {"fill"}
// Dependencies: {}
# [doc = " Fill the area with the specified symbol and style."] # [doc = ""] # [doc = " This probably should be a method on the `Buffer` type, but it is defined here for simplicity."] # [doc = " <https://github.com/ratatui/ratatui/issues/1146>"] fn fill < S : Into < Style > > (area : Rect , buf : & mut Buffer , symbol : & str , style : S) { let style = style . into () ; for y in area . top () .. area . bottom () { for x in area . left () .. area . right () { buf [(x , y)] . set_symbol (symbol) . set_style (style) ; } } }
};
}
