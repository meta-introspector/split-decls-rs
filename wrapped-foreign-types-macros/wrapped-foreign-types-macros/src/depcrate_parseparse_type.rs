// Generated macro for parse_type (function)
macro_rules! Depcrate_parseparse_type {
() => {
// Module: crate::parse
// Provides: {"parse_type"}
// Dependencies: {}
fn parse_type < T > (input : ParseStream) -> parse :: Result < Type > where T : Parse , { input . parse :: < Token ! [type] > () ? ; input . parse :: < T > () ? ; input . parse :: < Token ! [=] > () ? ; let type_ = input . parse () ? ; input . parse :: < Token ! [;] > () ? ; Ok (type_) }
};
}
