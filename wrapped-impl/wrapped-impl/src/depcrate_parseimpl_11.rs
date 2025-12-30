// Generated macro for impl_11 (impl)
macro_rules! Depcrate_parseimpl_11 {
() => {
// Module: crate::parse
// Provides: {"impl_11"}
// Dependencies: {}
impl Parse for ImplArgs { fn parse (input : ParseStream) -> Result < Self > { let name = if input . is_empty () { None } else { input . parse :: < kw :: name > () ? ; input . parse :: < Token ! [=] > () ? ; let name : LitStr = input . parse () ? ; input . parse :: < Option < Token ! [,] > > () ? ; Some (name) } ; Ok (ImplArgs { name }) } }
};
}
