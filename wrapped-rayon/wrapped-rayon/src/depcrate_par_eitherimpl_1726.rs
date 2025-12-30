// Generated macro for impl_1726 (impl)
macro_rules! Depcrate_par_eitherimpl_1726 {
() => {
// Module: crate::par_either
// Provides: {"impl_1726"}
// Dependencies: {}
# [doc = " `Either<L, R>` can be extended if both `L` and `R` are parallel extendable."] impl < L , R , T > ParallelExtend < T > for Either < L , R > where L : ParallelExtend < T > , R : ParallelExtend < T > , T : Send , { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = T > , { match self . as_mut () { Left (collection) => collection . par_extend (par_iter) , Right (collection) => collection . par_extend (par_iter) , } } }
};
}
