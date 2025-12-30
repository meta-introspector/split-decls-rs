// Generated macro for do_alert (function)
macro_rules! Depcratedo_alert {
() => {
// Module: crate
// Provides: {"do_alert"}
// Dependencies: {}
fn do_alert (color : Color , input : TokenStream) -> TokenStream { let message = parse_macro_input ! (input as LitStr) . value () ; let ref mut stderr = StandardStream :: stderr (ColorChoice :: Auto) ; let color_spec = ColorSpec :: new () . set_fg (Some (color)) . clone () ; let mut has_nonspace = false ; let mut says_error = false ; for mut line in message . lines () { if ! has_nonspace { let (indent , heading , rest) = split_heading (line) ; if let Some (indent) = indent { let _ = write ! (stderr , "{}" , indent) ; } if let Some (heading) = heading { let _ = stderr . set_color (color_spec . clone () . set_bold (true)) ; let _ = write ! (stderr , "{}" , heading) ; has_nonspace = true ; says_error = heading == "ERROR" ; } line = rest ; } if line . is_empty () { let _ = writeln ! (stderr) ; } else { let _ = stderr . set_color (& color_spec) ; let _ = writeln ! (stderr , "{}" , line) ; has_nonspace = has_nonspace || line . contains (| ch : char | ch != ' ') ; } } let _ = stderr . reset () ; let _ = writeln ! (stderr) ; if color == Color :: Red && says_error { process :: exit (1) ; } else { TokenStream :: new () } }
};
}
