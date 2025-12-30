// Generated macro for impl_632 (impl)
macro_rules! Depcrate_iter_find_first_lastimpl_632 {
() => {
// Module: crate::iter::find_first_last
// Provides: {"impl_632"}
// Dependencies: {}
impl < T > Reducer < Option < T > > for FindReducer { fn reduce (self , left : Option < T > , right : Option < T >) -> Option < T > { match self . match_position { MatchPosition :: Leftmost => left . or (right) , MatchPosition :: Rightmost => right . or (left) , } } }
};
}
