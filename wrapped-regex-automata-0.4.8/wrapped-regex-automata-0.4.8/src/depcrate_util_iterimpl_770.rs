// Generated macro for impl_770 (impl)
macro_rules! Depcrate_util_iterimpl_770 {
() => {
// Module: crate::util::iter
// Provides: {"impl_770"}
// Dependencies: {}
impl < 'h , F > Iterator for MatchesIter < 'h , F > where F : FnMut (& Input < '_ >) -> Result < Option < Match > , MatchError > , { type Item = Match ; # [inline] fn next (& mut self) -> Option < Match > { match self . 0 . next () ? { Ok (m) => Some (m) , Err (err) => panic ! ("unexpected regex find error: {}\n\
                 to handle find errors, use 'try' or 'search' methods" , err ,) , } } }
};
}
