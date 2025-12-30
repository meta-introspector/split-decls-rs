// Generated macro for parse_opt_const_directives (function)
macro_rules! Depcrate_parseparse_opt_const_directives {
() => {
// Module: crate::parse
// Provides: {"parse_opt_const_directives"}
// Dependencies: {}
fn parse_opt_const_directives (pairs : & mut Pairs < '_ , Rule > , pc : & mut PositionCalculator ,) -> Result < Vec < Positioned < ConstDirective > > > { Ok (parse_if_rule (pairs , Rule :: const_directives , | pair | { parse_const_directives (pair , pc) }) ? . unwrap_or_default ()) }
};
}
