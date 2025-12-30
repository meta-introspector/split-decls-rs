// Generated macro for impl_766 (impl)
macro_rules! Depcrate_util_iterimpl_766 {
() => {
// Module: crate::util::iter
// Provides: {"impl_766"}
// Dependencies: {}
impl < 'h , F > Iterator for HalfMatchesIter < 'h , F > where F : FnMut (& Input < '_ >) -> Result < Option < HalfMatch > , MatchError > , { type Item = HalfMatch ; # [inline] fn next (& mut self) -> Option < HalfMatch > { match self . 0 . next () ? { Ok (m) => Some (m) , Err (err) => panic ! ("unexpected regex half find error: {err}\n\
                 to handle find errors, use 'try' or 'search' methods" ,) , } } }
};
}
