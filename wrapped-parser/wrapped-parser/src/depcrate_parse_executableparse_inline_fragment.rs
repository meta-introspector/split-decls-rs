// Generated macro for parse_inline_fragment (function)
macro_rules! Depcrate_parse_executableparse_inline_fragment {
() => {
// Module: crate::parse::executable
// Provides: {"parse_inline_fragment"}
// Dependencies: {}
fn parse_inline_fragment (pair : Pair < Rule > , pc : & mut PositionCalculator , remaining_depth : usize ,) -> Result < Positioned < InlineFragment > > { debug_assert_eq ! (pair . as_rule () , Rule :: inline_fragment) ; let pos = pc . step (& pair) ; let mut pairs = pair . into_inner () ; let type_condition = parse_if_rule (& mut pairs , Rule :: type_condition , | pair | { parse_type_condition (pair , pc) }) ? ; let directives = parse_opt_directives (& mut pairs , pc) ? ; let selection_set = parse_selection_set (pairs . next () . unwrap () , pc , recursion_depth ! (remaining_depth)) ? ; debug_assert_eq ! (pairs . next () , None) ; Ok (Positioned :: new (InlineFragment { type_condition , directives , selection_set , } , pos ,)) }
};
}
