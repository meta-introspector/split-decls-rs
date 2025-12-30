// Generated macro for found (function)
macro_rules! Depcrate_syntax_textfound {
() => {
// Module: crate::syntax_text
// Provides: {"found"}
// Dependencies: {}
fn found < T > (res : Result < () , T >) -> Option < T > { match res { Ok (()) => None , Err (it) => Some (it) , } }
};
}
