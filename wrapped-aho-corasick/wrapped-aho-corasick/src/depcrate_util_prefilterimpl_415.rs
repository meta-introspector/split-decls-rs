// Generated macro for impl_415 (impl)
macro_rules! Depcrate_util_prefilterimpl_415 {
() => {
// Module: crate::util::prefilter
// Provides: {"impl_415"}
// Dependencies: {}
impl Candidate { # [doc = " Convert this candidate into an option. This is useful when callers"] # [doc = " do not distinguish between true positives and false positives (i.e.,"] # [doc = " the caller must always confirm the match)."] pub fn into_option (self) -> Option < usize > { match self { Candidate :: None => None , Candidate :: Match (ref m) => Some (m . start ()) , Candidate :: PossibleStartOfMatch (start) => Some (start) , } } }
};
}
