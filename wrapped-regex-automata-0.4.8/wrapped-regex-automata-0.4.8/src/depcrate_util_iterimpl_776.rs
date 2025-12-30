// Generated macro for impl_776 (impl)
macro_rules! Depcrate_util_iterimpl_776 {
() => {
// Module: crate::util::iter
// Provides: {"impl_776"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'h , F > Iterator for CapturesIter < 'h , F > where F : FnMut (& Input < '_ > , & mut Captures) -> Result < () , MatchError > , { type Item = Captures ; # [inline] fn next (& mut self) -> Option < Captures > { match self . 0 . next () ? { Ok (m) => Some (m) , Err (err) => panic ! ("unexpected regex captures error: {}\n\
                 to handle find errors, use 'try' or 'search' methods" , err ,) , } } }
};
}
