// Generated macro for parse_if_rule (function)
macro_rules! Depcrate_parse_utilsparse_if_rule {
() => {
// Module: crate::parse::utils
// Provides: {"parse_if_rule"}
// Dependencies: {}
pub (super) fn parse_if_rule < T > (pairs : & mut Pairs < Rule > , rule : Rule , f : impl FnOnce (Pair < Rule >) -> Result < T > ,) -> Result < Option < T > > { next_if_rule (pairs , rule) . map (f) . transpose () }
};
}
