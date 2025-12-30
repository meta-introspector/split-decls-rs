// Generated macro for impl_26 (impl)
macro_rules! Depcrate_attributesimpl_26 {
() => {
// Module: crate::attributes
// Provides: {"impl_26"}
// Dependencies: {}
impl Parse for AttrNameValue { fn parse (input : ParseStream < '_ >) -> syn :: Result < Self > { let name = match input . parse () { Ok (name) => name , Err (e) => { if let Ok (tok) = input . parse :: < Token ! [type] > () { Path :: from (Ident :: new ("type" , tok . span)) } else { return Err (e) ; } } } ; input . parse :: < Token ! [=] > () ? ; let value = input . parse () ? ; Ok (Self { name , value }) } }
};
}
