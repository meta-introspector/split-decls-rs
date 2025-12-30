// Generated macro for next_if_rule (function)
macro_rules! Depcrate_parse_utilsnext_if_rule {
() => {
// Module: crate::parse::utils
// Provides: {"next_if_rule"}
// Dependencies: {}
pub (super) fn next_if_rule < 'a > (pairs : & mut Pairs < 'a , Rule > , rule : Rule) -> Option < Pair < 'a , Rule > > { if pairs . peek () . is_some_and (| pair | pair . as_rule () == rule) { Some (pairs . next () . unwrap ()) } else { None } }
};
}
