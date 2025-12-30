// Generated macro for impl_21 (impl)
macro_rules! Depcrate_testimpl_21 {
() => {
// Module: crate::test
// Provides: {"impl_21"}
// Dependencies: {}
impl < T : Parse > Parse for Outer < T > { fn parse (input : syn :: parse :: ParseStream) -> syn :: Result < Self > { let outer : Ident = input . parse () ? ; if outer == "outer" { let content ; let _ = syn :: parenthesized ! (content in input) ; content . parse () . map (Outer) } else { Err (Error :: new (outer . span () , "Expected 'outer'")) } } }
};
}
