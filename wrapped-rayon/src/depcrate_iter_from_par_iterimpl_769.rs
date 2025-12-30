// Generated macro for impl_769 (impl)
macro_rules! Depcrate_iter_from_par_iterimpl_769 {
() => {
// Module: crate::iter::from_par_iter
// Provides: {"impl_769"}
// Dependencies: {}
# [doc = " Collects an arbitrary `Cow` collection."] # [doc = ""] # [doc = " Note, the standard library only has `FromIterator` for `Cow<'a, str>` and"] # [doc = " `Cow<'a, [T]>`, because no one thought to add a blanket implementation"] # [doc = " before it was stabilized."] impl < 'a , C , T > FromParallelIterator < T > for Cow < 'a , C > where C : ToOwned < Owned : FromParallelIterator < T > > + ? Sized , T : Send , { fn from_par_iter < I > (par_iter : I) -> Self where I : IntoParallelIterator < Item = T > , { Cow :: Owned (C :: Owned :: from_par_iter (par_iter)) } }
};
}
