// Generated macro for impl_42 (impl)
macro_rules! Depcrateimpl_42 {
() => {
// Module: crate
// Provides: {"impl_42"}
// Dependencies: {}
impl < B : BitBlock > Extend < bool > for BitVec < B > { # [inline] fn extend < I : IntoIterator < Item = bool > > (& mut self , iterable : I) { self . ensure_invariant () ; let iterator = iterable . into_iter () ; let (min , _) = iterator . size_hint () ; self . reserve (min) ; for element in iterator { self . push (element) } } }
};
}
