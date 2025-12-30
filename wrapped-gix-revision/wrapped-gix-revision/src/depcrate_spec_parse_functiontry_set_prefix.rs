// Generated macro for try_set_prefix (function)
macro_rules! Depcrate_spec_parse_functiontry_set_prefix {
() => {
// Module: crate::spec::parse::function
// Provides: {"try_set_prefix"}
// Dependencies: {}
fn try_set_prefix (delegate : & mut impl Delegate , hex_name : & BStr , hint : Option < delegate :: PrefixHint < '_ > >) -> Option < () > { gix_hash :: Prefix :: from_hex (hex_name . to_str () . expect ("hexadecimal only")) . ok () . and_then (| prefix | delegate . disambiguate_prefix (prefix , hint)) }
};
}
