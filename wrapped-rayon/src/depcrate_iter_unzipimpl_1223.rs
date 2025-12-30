// Generated macro for impl_1223 (impl)
macro_rules! Depcrate_iter_unzipimpl_1223 {
() => {
// Module: crate::iter::unzip
// Provides: {"impl_1223"}
// Dependencies: {}
impl < L , R , A , B > FromParallelIterator < Either < L , R > > for (A , B) where L : Send , R : Send , A : Send + FromParallelIterator < L > , B : Send + FromParallelIterator < R > , { fn from_par_iter < I > (pi : I) -> Self where I : IntoParallelIterator < Item = Either < L , R > > , { fn identity < T > (x : T) -> T { x } let (a , b) : (Collector < A > , Collector < B >) = pi . into_par_iter () . partition_map (identity) ; (a . result . unwrap () , b . result . unwrap ()) } }
};
}
