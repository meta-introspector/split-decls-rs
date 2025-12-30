// Generated macro for display_hint_unknown (function)
macro_rules! Depcrate_testsdisplay_hint_unknown {
() => {
// Module: crate::tests
// Provides: {"display_hint_unknown"}
// Dependencies: {}
# [test] fn display_hint_unknown () { assert_eq ! (parse_param (":unknown" , ParserMode :: ForwardsCompatible) , Ok (Param { index : None , ty : Type :: Format , hint : Some (DisplayHint :: Unknown ("unknown" . to_string ())) , })) ; }
};
}
