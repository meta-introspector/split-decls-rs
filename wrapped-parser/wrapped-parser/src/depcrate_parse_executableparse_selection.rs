// Generated macro for parse_selection (function)
macro_rules! Depcrate_parse_executableparse_selection {
() => {
// Module: crate::parse::executable
// Provides: {"parse_selection"}
// Dependencies: {}
fn parse_selection (pair : Pair < Rule > , pc : & mut PositionCalculator , remaining_depth : usize ,) -> Result < Positioned < Selection > > { debug_assert_eq ! (pair . as_rule () , Rule :: selection) ; let pos = pc . step (& pair) ; let pair = exactly_one (pair . into_inner ()) ; Ok (Positioned :: new (match pair . as_rule () { Rule :: field => Selection :: Field (parse_field (pair , pc , remaining_depth) ?) , Rule :: fragment_spread => Selection :: FragmentSpread (parse_fragment_spread (pair , pc) ?) , Rule :: inline_fragment => { Selection :: InlineFragment (parse_inline_fragment (pair , pc , remaining_depth) ?) } _ => unreachable ! () , } , pos ,)) }
};
}
