// Generated macro for parse_spec (function)
macro_rules! Depcrate_parserparse_spec {
() => {
// Module: crate::parser
// Provides: {"parse_spec"}
// Dependencies: {}
# [doc = " Parse a logging specification string (e.g: `crate1,crate2::mod3,crate3::x=error/foo`)"] # [doc = " and return a vector with log directives."] pub (crate) fn parse_spec (spec : & str) -> ParseResult { let mut result = ParseResult :: default () ; let mut parts = spec . split ('/') ; let mods = parts . next () ; let filter = parts . next () ; if parts . next () . is_some () { result . add_error (format ! ("invalid logging spec '{spec}' (too many '/'s)")) ; return result ; } if let Some (m) = mods { for s in m . split (',') . map (| ss | ss . trim ()) { if s . is_empty () { continue ; } let mut parts = s . split ('=') ; let (log_level , name) = match (parts . next () , parts . next () . map (| s | s . trim ()) , parts . next ()) { (Some (part0) , None , None) => { match part0 . parse () { Ok (num) => (num , None) , Err (_) => (LevelFilter :: max () , Some (part0)) , } } (Some (part0) , Some ("") , None) => (LevelFilter :: max () , Some (part0)) , (Some (part0) , Some (part1) , None) => { if let Ok (num) = part1 . parse () { (num , Some (part0)) } else { result . add_error (format ! ("invalid logging spec '{part1}'")) ; continue ; } } _ => { result . add_error (format ! ("invalid logging spec '{s}'")) ; continue ; } } ; result . add_directive (Directive { name : name . map (| s | s . to_owned ()) , level : log_level , }) ; } } if let Some (filter) = filter { match FilterOp :: new (filter) { Ok (filter_op) => result . set_filter (filter_op) , Err (err) => result . add_error (format ! ("invalid regex filter - {err}")) , } } result }
};
}
