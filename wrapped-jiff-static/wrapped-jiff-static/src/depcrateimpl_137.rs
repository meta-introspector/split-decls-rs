// Generated macro for impl_137 (impl)
macro_rules! Depcrateimpl_137 {
() => {
// Module: crate
// Provides: {"impl_137"}
// Dependencies: {}
# [cfg (feature = "tzdb")] impl syn :: parse :: Parse for Get { fn parse (input : syn :: parse :: ParseStream) -> syn :: Result < Get > { let lit1 = input . parse :: < syn :: LitStr > () ? . value () ; if input . lookahead1 () . peek (syn :: Token ! [,]) { input . parse :: < syn :: Token ! [,] > () ? ; } Ok (Get :: from_id (& lit1) . map_err (| e | input . error (e)) ?) } }
};
}
