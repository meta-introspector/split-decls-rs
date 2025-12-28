macro_rules! term_supports_color {
    () => {
        # [doc = " Check `TERM` for color support"] # [inline] pub fn term_supports_color () -> bool { # [cfg (not (windows))] { match std :: env :: var_os ("TERM") { None => return false , Some (k) => { if k == "dumb" { return false ; } } } true } # [cfg (windows)] { if let Some (k) = std :: env :: var_os ("TERM") { if k == "dumb" { return false ; } } true } }
    };
}

term_supports_color!();