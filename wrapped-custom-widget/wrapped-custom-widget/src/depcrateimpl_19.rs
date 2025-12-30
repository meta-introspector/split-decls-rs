// Generated macro for impl_19 (impl)
macro_rules! Depcrateimpl_19 {
() => {
// Module: crate
// Provides: {"impl_19"}
// Dependencies: {}
impl Widget for Button < '_ > { # [expect (clippy :: cast_possible_truncation)] fn render (self , area : Rect , buf : & mut Buffer) { let (background , text , shadow , highlight) = self . colors () ; buf . set_style (area , Style :: new () . bg (background) . fg (text)) ; if area . height > 2 { buf . set_string (area . x , area . y , "▔" . repeat (area . width as usize) , Style :: new () . fg (highlight) . bg (background) ,) ; } if area . height > 1 { buf . set_string (area . x , area . y + area . height - 1 , "▁" . repeat (area . width as usize) , Style :: new () . fg (shadow) . bg (background) ,) ; } buf . set_line (area . x + (area . width . saturating_sub (self . label . width () as u16)) / 2 , area . y + (area . height . saturating_sub (1)) / 2 , & self . label , area . width ,) ; } }
};
}
