// Generated macro for invalid (macro)
macro_rules! Depcrate_v0invalid {
() => {
// Module: crate::v0
// Provides: {"invalid"}
// Dependencies: {}
# [doc = " Mark the parser as errored (with `ParseError::Invalid`), print the"] # [doc = " appropriate message (see `ParseError::message`) and return early."] macro_rules ! invalid { ($ printer : ident) => { { let err = ParseError :: Invalid ; $ printer . print (err . message ()) ?; $ printer . parser = Err (err) ; return Ok (()) ; } } ; }
};
}
