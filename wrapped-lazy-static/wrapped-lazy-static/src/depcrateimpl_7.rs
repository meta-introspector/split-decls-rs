// Generated macro for impl_7 (impl)
macro_rules! Depcrateimpl_7 {
() => {
// Module: crate
// Provides: {"impl_7"}
// Dependencies: {}
impl Parse for LazyStatic { fn parse (input : ParseStream) -> Result < Self > { let visibility : Visibility = input . parse () ? ; input . parse :: < Token ! [static] > () ? ; input . parse :: < Token ! [ref] > () ? ; let name : Ident = input . parse () ? ; input . parse :: < Token ! [:] > () ? ; let ty : Type = input . parse () ? ; input . parse :: < Token ! [=] > () ? ; let init : Expr = input . parse () ? ; input . parse :: < Token ! [;] > () ? ; Ok (LazyStatic { visibility , name , ty , init , }) } }
};
}
