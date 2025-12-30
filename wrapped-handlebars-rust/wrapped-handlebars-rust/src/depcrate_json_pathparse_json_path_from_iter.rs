// Generated macro for parse_json_path_from_iter (function)
macro_rules! Depcrate_json_pathparse_json_path_from_iter {
() => {
// Module: crate::json::path
// Provides: {"parse_json_path_from_iter"}
// Dependencies: {}
pub (crate) fn parse_json_path_from_iter < 'a , I > (it : & mut Peekable < I > , limit : usize) -> Vec < PathSeg > where I : Iterator < Item = Pair < 'a , Rule > > , { let mut path_stack = Vec :: with_capacity (5) ; while let Some (n) = it . peek () { let span = n . as_span () ; if span . end () > limit { break ; } match n . as_rule () { Rule :: path_root => { path_stack . push (PathSeg :: Ruled (Rule :: path_root)) ; } Rule :: path_local => { path_stack . push (PathSeg :: Ruled (Rule :: path_local)) ; } Rule :: path_up => { path_stack . push (PathSeg :: Ruled (Rule :: path_up)) ; } Rule :: path_id | Rule :: path_raw_id => { let name = n . as_str () ; if name != "this" { path_stack . push (PathSeg :: Named (name . to_string ())) ; } } _ => { } } it . next () ; } path_stack }
};
}
