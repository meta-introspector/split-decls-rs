// Generated macro for parse_opt_directives (function)
macro_rules! Depcrate_parseparse_opt_directives {
() => {
// Module: crate::parse
// Provides: {"parse_opt_directives"}
// Dependencies: {}
fn parse_opt_directives (pairs : & mut Pairs < '_ , Rule > , pc : & mut PositionCalculator ,) -> Result < Vec < Positioned < Directive > > > { Ok (parse_if_rule (pairs , Rule :: directives , | pair | parse_directives (pair , pc)) ? . unwrap_or_default () ,) }
};
}
