// Generated macro for parse_directives (function)
macro_rules! Depcrate_parseparse_directives {
() => {
// Module: crate::parse
// Provides: {"parse_directives"}
// Dependencies: {}
fn parse_directives (pair : Pair < Rule > , pc : & mut PositionCalculator ,) -> Result < Vec < Positioned < Directive > > > { debug_assert_eq ! (pair . as_rule () , Rule :: directives) ; pair . into_inner () . map (| pair | parse_directive (pair , pc)) . collect () }
};
}
