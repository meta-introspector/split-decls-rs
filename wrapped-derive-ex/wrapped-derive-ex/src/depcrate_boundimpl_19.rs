// Generated macro for impl_19 (impl)
macro_rules! Depcrate_boundimpl_19 {
() => {
// Module: crate::bound
// Provides: {"impl_19"}
// Dependencies: {}
impl Parse for Bound { fn parse (input : ParseStream) -> Result < Self > { if input . peek (Token ! [..]) { return Ok (Self :: Default (input . parse () ?)) ; } let fork = input . fork () ; match fork . parse () { Ok (p) => { input . advance_to (& fork) ; Ok (Self :: Pred (p)) } Err (e) => { if let Ok (ty) = input . parse () { Ok (Self :: Type (ty)) } else { Err (e) } } } } }
};
}
