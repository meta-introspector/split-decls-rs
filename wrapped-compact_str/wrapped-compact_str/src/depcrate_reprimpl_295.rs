// Generated macro for impl_295 (impl)
macro_rules! Depcrate_reprimpl_295 {
() => {
// Module: crate::repr
// Provides: {"impl_295"}
// Dependencies: {}
impl Extend < char > for Repr { # [inline] fn extend < T : IntoIterator < Item = char > > (& mut self , iter : T) { let iter = iter . into_iter () ; let (lower_bound , _) = iter . size_hint () ; if lower_bound > 0 { let _ : Result < () , ReserveError > = self . reserve (lower_bound) ; } for c in iter { self . push_str (c . encode_utf8 (& mut [0 ; 4])) ; } } }
};
}
