// Generated macro for impl_808 (impl)
macro_rules! Depcrate_prefixesimpl_808 {
() => {
// Module: crate::prefixes
// Provides: {"impl_808"}
// Dependencies: {}
impl < 'tcx > MirBorrowckCtxt < '_ , '_ , 'tcx > { # [doc = " Returns an iterator over the prefixes of `place`"] # [doc = " (inclusive) from longest to smallest, potentially"] # [doc = " terminating the iteration early based on `kind`."] pub (super) fn prefixes (& self , place_ref : PlaceRef < 'tcx > , kind : PrefixSet) -> Prefixes < 'tcx > { Prefixes { next : Some (place_ref) , kind } } }
};
}
