// Generated macro for impl_9 (impl)
macro_rules! Depcrate_ansiimpl_9 {
() => {
// Module: crate::ansi
// Provides: {"impl_9"}
// Dependencies: {}
impl Style { # [doc = " Write any bytes that go *before* a piece of text to the given writer."] fn write_prefix < W : AnyWrite + ? Sized > (& self , f : & mut W) -> Result < () , W :: Error > { if self . is_plain () { return Ok (()) ; } write ! (f , "\x1B[") ? ; let mut written_anything = false ; { let mut write_char = | c | { if written_anything { write ! (f , ";") ? ; } written_anything = true ; # [cfg (feature = "gnu_legacy")] write ! (f , "0") ? ; write ! (f , "{}" , c) ? ; Ok (()) } ; if self . is_bold { write_char ('1') ? } if self . is_dimmed { write_char ('2') ? } if self . is_italic { write_char ('3') ? } if self . is_underline { write_char ('4') ? } if self . is_blink { write_char ('5') ? } if self . is_reverse { write_char ('7') ? } if self . is_hidden { write_char ('8') ? } if self . is_strikethrough { write_char ('9') ? } } if let Some (bg) = self . background { if written_anything { write ! (f , ";") ? ; } written_anything = true ; bg . write_background_code (f) ? ; } if let Some (fg) = self . foreground { if written_anything { write ! (f , ";") ? ; } fg . write_foreground_code (f) ? ; } write ! (f , "m") ? ; Ok (()) } # [doc = " Write any bytes that go *after* a piece of text to the given writer."] fn write_suffix < W : AnyWrite + ? Sized > (& self , f : & mut W) -> Result < () , W :: Error > { if self . is_plain () { Ok (()) } else { write ! (f , "{}" , RESET) } } }
};
}
