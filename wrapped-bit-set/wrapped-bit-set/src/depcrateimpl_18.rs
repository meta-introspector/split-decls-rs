// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
impl < B : BitBlock > Extend < usize > for BitSet < B > { # [inline] fn extend < I : IntoIterator < Item = usize > > (& mut self , iter : I) { for i in iter { self . insert (i) ; } } }
};
}
