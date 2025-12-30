// Generated macro for parse_phantom_data (function)
macro_rules! Depcrate_parseparse_phantom_data {
() => {
// Module: crate::parse
// Provides: {"parse_phantom_data"}
// Dependencies: {}
fn parse_phantom_data (input : ParseStream) -> parse :: Result < Option < Type > > { if input . peek (Token ! [type]) && input . peek2 (kw :: PhantomData) { input . call (parse_type :: < kw :: PhantomData >) . map (Some) } else { Ok (None) } }
};
}
