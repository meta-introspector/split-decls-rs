// Generated macro for impl_1219 (impl)
macro_rules! Depcrate_iter_unzipimpl_1219 {
() => {
// Module: crate::iter::unzip
// Provides: {"impl_1219"}
// Dependencies: {}
impl < L , R , A , B > ParallelExtend < Either < L , R > > for (A , B) where L : Send , R : Send , A : Send + ParallelExtend < L > , B : Send + ParallelExtend < R > , { fn par_extend < I > (& mut self , pi : I) where I : IntoParallelIterator < Item = Either < L , R > > , { execute_into (& mut self . 0 , & mut self . 1 , pi . into_par_iter () , UnEither) ; } }
};
}
