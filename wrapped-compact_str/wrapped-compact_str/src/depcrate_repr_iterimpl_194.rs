// Generated macro for impl_194 (impl)
macro_rules! Depcrate_repr_iterimpl_194 {
() => {
// Module: crate::repr::iter
// Provides: {"impl_194"}
// Dependencies: {}
impl FromIterator < char > for Repr { # [inline] fn from_iter < T : IntoIterator < Item = char > > (iter : T) -> Self { let iter = iter . into_iter () ; let (lower_bound , _) = iter . size_hint () ; let mut this = match Repr :: with_capacity (lower_bound) { Ok (this) => this , Err (_) => EMPTY , } ; for c in iter { this . push_str (c . encode_utf8 (& mut [0 ; 4])) ; } this } }
};
}
