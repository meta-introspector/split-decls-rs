// Generated macro for try_parse (function)
macro_rules! Depcrate_argstry_parse {
() => {
// Module: crate::args
// Provides: {"try_parse"}
// Dependencies: {}
fn try_parse (input : ParseStream) -> Result < Args > { if input . peek (Token ! [?]) { input . parse :: < Token ! [?] > () ? ; input . parse :: < kw :: Send > () ? ; Ok (Args { local : true }) } else { Ok (Args { local : false }) } }
};
}
