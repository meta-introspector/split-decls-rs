// Generated macro for impl_426 (impl)
macro_rules! Depcrate_hash_setimpl_426 {
() => {
// Module: crate::hash::set
// Provides: {"impl_426"}
// Dependencies: {}
impl < 's , 'a , A , OA , SA , SB > From < & 's HashSet < & 'a A , SA > > for HashSet < OA , SB > where A : ToOwned < Owned = OA > + Hash + Eq + ? Sized , OA : Borrow < A > + Hash + Eq + Clone , SA : BuildHasher , SB : BuildHasher + Default , { fn from (set : & HashSet < & A , SA >) -> Self { set . iter () . map (| a | (* a) . to_owned ()) . collect () } }
};
}
