// Generated macro for add_color_to_roff (function)
macro_rules! Depcrateadd_color_to_roff {
() => {
// Module: crate
// Provides: {"add_color_to_roff"}
// Dependencies: {}
fn add_color_to_roff (doc : & mut Roff , control_request : & str , color : & Option < Color >) { match color { Some (Color :: Rgb (c)) => { let name = rgb_name (c) ; doc . control (control_requests :: CREATE_COLOR , [name . as_str () , "rgb" , to_hex (c) . as_str ()] ,) . control (control_request , [name . as_str ()]) ; } Some (Color :: Ansi (c)) => { doc . control (control_request , [ansi_color_to_roff (c)]) ; } Some (Color :: Ansi256 (c)) => { add_color_to_roff (doc , control_request , & Some (xterm_to_ansi_or_rgb (* c))) ; } None => { doc . control (control_request , ["default"]) ; } } }
};
}
