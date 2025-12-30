// Generated macro for impl_99 (impl)
macro_rules! Depcrate_astimpl_99 {
() => {
// Module: crate::ast
// Provides: {"impl_99"}
// Dependencies: {}
impl < 'a > TryFrom < Pair < 'a , Rule > > for ID < 'a > { type Error = ParseError < 'a > ; fn try_from (p : Pair < 'a , Rule >) -> Result < Self , ParseError < 'a > > { let inner = p . clone () . into_inner () . next () . ok_or (ParseError :: missing_pair (p . clone () , vec ! [Rule :: ident1 , Rule :: numeral , Rule :: quote , Rule :: html] ,)) ? ; let id = match inner . as_rule () { Rule :: ident1 => ID (inner . as_str ()) , Rule :: numeral => ID (inner . as_str ()) , Rule :: quote => { let mut inner = inner . into_inner () ; inner . next () . ok_or (ParseError :: missing_pair (p . clone () , vec ! [Rule :: quotemark])) ? ; let text = inner . next () . ok_or (ParseError :: missing_pair (p . clone () , vec ! [Rule :: quote_escaped] ,)) ? ; ID (text . as_str ()) } Rule :: html => ID (inner . as_str ()) , _ => Err (ParseError :: expect_rule (vec ! [Rule :: ident1 , Rule :: numeral , Rule :: quote , Rule :: html] , p . as_rule () ,)) ? , } ; Ok (id) } }
};
}
