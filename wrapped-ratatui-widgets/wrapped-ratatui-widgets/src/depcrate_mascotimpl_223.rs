// Generated macro for impl_223 (impl)
macro_rules! Depcrate_mascotimpl_223 {
() => {
// Module: crate::mascot
// Provides: {"impl_223"}
// Dependencies: {}
impl Widget for RatatuiMascot { # [doc = " Use half block characters to render a logo based on the `RATATUI_LOGO` const."] # [doc = ""] # [doc = " The logo colors are hardcorded in the widget."] # [doc = " The eye color depends on whether it's open / blinking"] fn render (self , area : Rect , buf : & mut Buffer) { let area = area . intersection (buf . area) ; if area . is_empty () { return ; } for (y , (line1 , line2)) in RATATUI_MASCOT . lines () . tuples () . enumerate () { for (x , (ch1 , ch2)) in line1 . chars () . zip (line2 . chars ()) . enumerate () { let x = area . left () + x as u16 ; let y = area . top () + y as u16 ; if x >= area . right () || y >= area . bottom () { continue ; } let cell = & mut buf [(x , y)] ; let (fg , bg) = match (ch1 , ch2) { (EMPTY , EMPTY) => (None , None) , (c , EMPTY) | (EMPTY , c) => (self . color_for (c) , None) , (TERM , TERM_BORDER) => (self . color_for (TERM_BORDER) , self . color_for (TERM)) , (TERM , c) | (c , TERM) => (self . color_for (c) , self . color_for (TERM)) , (c1 , c2) => (self . color_for (c1) , self . color_for (c2)) , } ; let symbol = match (ch1 , ch2) { (EMPTY , EMPTY) => None , (TERM , TERM) => Some (EMPTY) , (_ , EMPTY | TERM) => Some ('▀') , (EMPTY | TERM , _) => Some ('▄') , (c , d) if c == d => Some ('█') , (_ , _) => Some ('▀') , } ; if let Some (fg) = fg { cell . fg = fg ; } if let Some (bg) = bg { cell . bg = bg ; } if let Some (symb) = symbol { cell . set_char (symb) ; } } } } }
};
}
