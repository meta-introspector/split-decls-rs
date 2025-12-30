// Generated macro for parse_const_directives (function)
macro_rules! Depcrate_parseparse_const_directives {
() => {
// Module: crate::parse
// Provides: {"parse_const_directives"}
// Dependencies: {}
fn parse_const_directives (pair : Pair < Rule > , pc : & mut PositionCalculator ,) -> Result < Vec < Positioned < ConstDirective > > > { debug_assert_eq ! (pair . as_rule () , Rule :: const_directives) ; pair . into_inner () . map (| pair | parse_const_directive (pair , pc)) . collect () }
};
}
