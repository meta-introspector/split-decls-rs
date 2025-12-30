// Generated macro for parse_field (function)
macro_rules! Depcrate_parse_executableparse_field {
() => {
// Module: crate::parse::executable
// Provides: {"parse_field"}
// Dependencies: {}
fn parse_field (pair : Pair < Rule > , pc : & mut PositionCalculator , remaining_depth : usize ,) -> Result < Positioned < Field > > { debug_assert_eq ! (pair . as_rule () , Rule :: field) ; let pos = pc . step (& pair) ; let mut pairs = pair . into_inner () ; let alias = parse_if_rule (& mut pairs , Rule :: alias , | pair | parse_alias (pair , pc)) ? ; let name = parse_name (pairs . next () . unwrap () , pc) ? ; let arguments = parse_if_rule (& mut pairs , Rule :: arguments , | pair | { parse_arguments (pair , pc) }) ? ; let directives = parse_opt_directives (& mut pairs , pc) ? ; let selection_set = parse_if_rule (& mut pairs , Rule :: selection_set , | pair | { parse_selection_set (pair , pc , recursion_depth ! (remaining_depth)) }) ? ; debug_assert_eq ! (pairs . next () , None) ; Ok (Positioned :: new (Field { alias , name , arguments : arguments . unwrap_or_default () , directives , selection_set : selection_set . unwrap_or_default () , } , pos ,)) }
};
}
