// Generated macro for impl_773 (impl)
macro_rules! Depcrate_util_iterimpl_773 {
() => {
// Module: crate::util::iter
// Provides: {"impl_773"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'h , F > Iterator for TryCapturesIter < 'h , F > where F : FnMut (& Input < '_ > , & mut Captures) -> Result < () , MatchError > , { type Item = Result < Captures , MatchError > ; # [inline] fn next (& mut self) -> Option < Result < Captures , MatchError > > { let TryCapturesIter { ref mut it , ref mut caps , ref mut finder } = * self ; let result = it . try_advance (| input | { (finder) (input , caps) ? ; Ok (caps . get_match ()) }) . transpose () ? ; match result { Ok (_) => Some (Ok (caps . clone ())) , Err (err) => Some (Err (err)) , } } }
};
}
