// Generated macro for impl_134 (impl)
macro_rules! Depcrateimpl_134 {
() => {
// Module: crate
// Provides: {"impl_134"}
// Dependencies: {}
impl syn :: parse :: Parse for Include { fn parse (input : syn :: parse :: ParseStream) -> syn :: Result < Include > { let lit1 = input . parse :: < syn :: LitStr > () ? . value () ; if ! input . lookahead1 () . peek (syn :: Token ! [,]) { return Ok (Include :: from_path_only (& lit1) . map_err (| e | input . error (e)) ?) ; } input . parse :: < syn :: Token ! [,] > () ? ; if input . is_empty () { return Ok (Include :: from_path_only (& lit1) . map_err (| e | input . error (e)) ?) ; } let lit2 = input . parse :: < syn :: LitStr > () ? . value () ; if input . lookahead1 () . peek (syn :: Token ! [,]) { input . parse :: < syn :: Token ! [,] > () ? ; } Ok (Include :: from_path_with_id (& lit2 , & lit1) . map_err (| e | input . error (e)) ?) } }
};
}
