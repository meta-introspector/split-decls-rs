// Generated macro for impl_64 (impl)
macro_rules! Depcrateimpl_64 {
() => {
// Module: crate
// Provides: {"impl_64"}
// Dependencies: {}
impl < T > Extend < T > for Arena < T > { fn extend < II : IntoIterator < Item = T > > (& mut self , iter : II) { for t in iter { self . alloc (t) ; } } }
};
}
