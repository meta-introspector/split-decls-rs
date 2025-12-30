// Generated macro for impl_769 (impl)
macro_rules! Depcrate_zip_longestimpl_769 {
() => {
// Module: crate::zip_longest
// Provides: {"impl_769"}
// Dependencies: {}
impl < T , U > DoubleEndedIterator for ZipLongest < T , U > where T : DoubleEndedIterator + ExactSizeIterator , U : DoubleEndedIterator + ExactSizeIterator , { # [inline] fn next_back (& mut self) -> Option < Self :: Item > { match self . a . len () . cmp (& self . b . len ()) { Equal => match (self . a . next_back () , self . b . next_back ()) { (None , None) => None , (Some (a) , Some (b)) => Some (EitherOrBoth :: Both (a , b)) , (Some (a) , None) => Some (EitherOrBoth :: Left (a)) , (None , Some (b)) => Some (EitherOrBoth :: Right (b)) , } , Greater => self . a . next_back () . map (EitherOrBoth :: Left) , Less => self . b . next_back () . map (EitherOrBoth :: Right) , } } fn rfold < B , F > (self , mut init : B , mut f : F) -> B where F : FnMut (B , Self :: Item) -> B , { let Self { mut a , mut b } = self ; let a_len = a . len () ; let b_len = b . len () ; match a_len . cmp (& b_len) { Equal => { } Greater => { init = a . by_ref () . rev () . take (a_len - b_len) . map (EitherOrBoth :: Left) . fold (init , & mut f) } Less => { init = b . by_ref () . rev () . take (b_len - a_len) . map (EitherOrBoth :: Right) . fold (init , & mut f) } } a . rfold (init , | acc , item_a | { f (acc , EitherOrBoth :: Both (item_a , b . next_back () . unwrap ())) }) } }
};
}
