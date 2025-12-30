// Generated macro for impl_24 (impl)
macro_rules! Depcrate_matches_patternimpl_24 {
() => {
// Module: crate::matches_pattern
// Provides: {"impl_24"}
// Dependencies: {}
impl Parse for MaybeTupleFieldPattern { fn parse (input : ParseStream) -> syn :: Result < Self > { let pattern = match input . parse :: < Option < Token ! [_] > > () ? { Some (_) => None , None => Some (TupleFieldPattern { ref_token : input . parse () ? , matcher : input . parse () ? }) , } ; Ok (MaybeTupleFieldPattern (pattern)) } }
};
}
