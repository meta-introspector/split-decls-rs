// Generated macro for impl_83 (impl)
macro_rules! Depcrate_dfaimpl_83 {
() => {
// Module: crate::dfa
// Provides: {"impl_83"}
// Dependencies: {}
impl < T > Result < T > { # [doc = " Returns true if this result corresponds to a match."] pub fn is_match (& self) -> bool { match * self { Result :: Match (_) => true , Result :: NoMatch (_) | Result :: Quit => false , } } # [doc = " Maps the given function onto T and returns the result."] # [doc = ""] # [doc = " If this isn't a match, then this is a no-op."] pub fn map < U , F : FnMut (T) -> U > (self , mut f : F) -> Result < U > { match self { Result :: Match (t) => Result :: Match (f (t)) , Result :: NoMatch (x) => Result :: NoMatch (x) , Result :: Quit => Result :: Quit , } } # [doc = " Sets the non-match position."] # [doc = ""] # [doc = " If this isn't a non-match, then this is a no-op."] fn set_non_match (self , at : usize) -> Result < T > { match self { Result :: NoMatch (_) => Result :: NoMatch (at) , r => r , } } }
};
}
