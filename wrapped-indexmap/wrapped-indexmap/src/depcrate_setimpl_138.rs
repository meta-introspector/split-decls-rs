// Generated macro for impl_138 (impl)
macro_rules! Depcrate_setimpl_138 {
() => {
// Module: crate::set
// Provides: {"impl_138"}
// Dependencies: {}
impl < 'a , T , S > Extend < & 'a T > for IndexSet < T , S > where T : Hash + Eq + Copy + 'a , S : BuildHasher , { fn extend < I : IntoIterator < Item = & 'a T > > (& mut self , iterable : I) { let iter = iterable . into_iter () . copied () ; self . extend (iter) ; } }
};
}
