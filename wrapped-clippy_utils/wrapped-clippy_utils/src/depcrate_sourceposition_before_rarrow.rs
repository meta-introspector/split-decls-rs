// Generated macro for position_before_rarrow (function)
macro_rules! Depcrate_sourceposition_before_rarrow {
() => {
// Module: crate::source
// Provides: {"position_before_rarrow"}
// Dependencies: {}
# [doc = " Returns the position just before rarrow"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " fn into(self) -> () {}"] # [doc = "              ^"] # [doc = " // in case of unformatted code"] # [doc = " fn into2(self)-> () {}"] # [doc = "               ^"] # [doc = " fn into3(self)   -> () {}"] # [doc = "               ^"] # [doc = " ```"] pub fn position_before_rarrow (s : & str) -> Option < usize > { s . rfind ("->") . map (| rpos | { let mut rpos = rpos ; let chars : Vec < char > = s . chars () . collect () ; while rpos > 1 { if let Some (c) = chars . get (rpos - 1) && c . is_whitespace () { rpos -= 1 ; continue ; } break ; } rpos }) }
};
}
