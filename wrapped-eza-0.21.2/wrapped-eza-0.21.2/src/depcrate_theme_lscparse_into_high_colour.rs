// Generated macro for parse_into_high_colour (function)
macro_rules! Depcrate_theme_lscparse_into_high_colour {
() => {
// Module: crate::theme::lsc
// Provides: {"parse_into_high_colour"}
// Dependencies: {}
fn parse_into_high_colour < 'a , I > (iter : & mut Peekable < I >) -> Option < Colour > where I : Iterator < Item = & 'a str > , { match iter . peek () { Some (& "5") => { let _ = iter . next () ; if let Some (byte) = iter . next () { if let Ok (num) = byte . parse () { return Some (Fixed (num)) ; } } } Some (& "2") => { let _ = iter . next () ; if let Some (hexes) = iter . next () { if let (Some (r) , Some (g) , Some (b)) = (hexes . parse () . ok () , iter . next () . and_then (| s | s . parse () . ok ()) , iter . next () . and_then (| s | s . parse () . ok ()) ,) { return Some (Rgb (r , g , b)) ; } } } _ => { } } None }
};
}
