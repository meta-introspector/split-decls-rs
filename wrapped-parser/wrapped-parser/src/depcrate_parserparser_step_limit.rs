// Generated macro for PARSER_STEP_LIMIT (const)
macro_rules! Depcrate_parserPARSER_STEP_LIMIT {
() => {
// Module: crate::parser
// Provides: {"PARSER_STEP_LIMIT"}
// Dependencies: {}
const PARSER_STEP_LIMIT : usize = if cfg ! (debug_assertions) { 150_000 } else { 15_000_000 } ;
};
}
