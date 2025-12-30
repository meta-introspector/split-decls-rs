// Generated macro for impl_630 (impl)
macro_rules! Depcrate_iter_find_first_lastimpl_630 {
() => {
// Module: crate::iter::find_first_last
// Provides: {"impl_630"}
// Dependencies: {}
impl < 'p , P : 'p + Fn (& T) -> bool , T > Folder < T > for FindFolder < 'p , T , P > { type Result = Option < T > ; fn consume (mut self , item : T) -> Self { let found_best_in_range = match self . match_position { MatchPosition :: Leftmost => self . item . is_some () , MatchPosition :: Rightmost => false , } ; if ! found_best_in_range && (self . find_op) (& item) { let update = self . best_found . fetch_update (Ordering :: Relaxed , Ordering :: Relaxed , | current | { better_position (self . boundary , current , self . match_position) . then_some (self . boundary) }) ; if update . is_ok () || update == Err (self . boundary) { self . item = Some (item) ; } } self } fn complete (self) -> Self :: Result { self . item } fn full (& self) -> bool { let found_best_in_range = match self . match_position { MatchPosition :: Leftmost => self . item . is_some () , MatchPosition :: Rightmost => false , } ; found_best_in_range || better_position (self . best_found . load (Ordering :: Relaxed) , self . boundary , self . match_position ,) } }
};
}
