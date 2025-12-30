// Generated macro for write_bg_span (function)
macro_rules! Depcratewrite_bg_span {
() => {
// Module: crate
// Provides: {"write_bg_span"}
// Dependencies: {}
fn write_bg_span (buffer : & mut String , span : & str , style : & anstyle :: Style , fragment : & str) { use std :: fmt :: Write as _ ; use unicode_width :: UnicodeWidthStr ; let bg_color = style . get_bg_color () . map (| c | color_name (BG_PREFIX , c)) ; let fill = if bg_color . is_some () { "█" } else { " " } ; let fragment = html_escape :: encode_text (fragment) ; let width = fragment . width () ; let fragment = fill . repeat (width) ; let mut classes = Vec :: new () ; if let Some (class) = bg_color . as_deref () { classes . push (class) ; } write ! (buffer , r#"<{span}"#) . unwrap () ; if ! classes . is_empty () { let classes = classes . join (" ") ; write ! (buffer , r#" class="{classes}""#) . unwrap () ; } write ! (buffer , r#">"#) . unwrap () ; write ! (buffer , "{fragment}") . unwrap () ; write ! (buffer , r#"</{span}>"#) . unwrap () ; }
};
}
