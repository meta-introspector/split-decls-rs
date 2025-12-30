// Generated macro for impl_492 (impl)
macro_rules! Depcrate_setimpl_492 {
() => {
// Module: crate::set
// Provides: {"impl_492"}
// Dependencies: {}
impl < 'a , T , S , A > Extend < & 'a T > for HashSet < T , S , A > where T : 'a + Eq + Hash + Copy , S : BuildHasher , A : Allocator , { # [cfg_attr (feature = "inline-more" , inline)] fn extend < I : IntoIterator < Item = & 'a T > > (& mut self , iter : I) { self . extend (iter . into_iter () . copied ()) ; } # [inline] # [cfg (feature = "nightly")] fn extend_one (& mut self , k : & 'a T) { self . map . insert (* k , ()) ; } # [inline] # [cfg (feature = "nightly")] fn extend_reserve (& mut self , additional : usize) { Extend :: < (T , ()) > :: extend_reserve (& mut self . map , additional) ; } }
};
}
