// Generated macro for impl_39 (impl)
macro_rules! Depcrate_arbitraryimpl_39 {
() => {
// Module: crate::arbitrary
// Provides: {"impl_39"}
// Dependencies: {}
impl < A : Arbitrary > VecShrinker < A > { # [allow (clippy :: new_ret_no_self)] fn new (seed : Vec < A >) -> Box < dyn Iterator < Item = Vec < A > > > { let es = match seed . first () { Some (e) => e . shrink () , None => return empty_shrinker () , } ; let size = seed . len () ; Box :: new (VecShrinker { seed , size , offset : size , element_shrinker : es , }) } # [doc = " Returns the next shrunk element if any, `offset` points to the index"] # [doc = " after the returned element after the function returns"] fn next_element (& mut self) -> Option < A > { loop { match self . element_shrinker . next () { Some (e) => return Some (e) , None => match self . seed . get (self . offset) { Some (e) => { self . element_shrinker = e . shrink () ; self . offset += 1 ; } None => return None , } , } } } }
};
}
