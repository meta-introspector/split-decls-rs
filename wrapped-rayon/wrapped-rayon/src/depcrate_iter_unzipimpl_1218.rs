// Generated macro for impl_1218 (impl)
macro_rules! Depcrate_iter_unzipimpl_1218 {
() => {
// Module: crate::iter::unzip
// Provides: {"impl_1218"}
// Dependencies: {}
impl < A , B , FromA , FromB > ParallelExtend < (A , B) > for (FromA , FromB) where A : Send , B : Send , FromA : Send + ParallelExtend < A > , FromB : Send + ParallelExtend < B > , { fn par_extend < I > (& mut self , pi : I) where I : IntoParallelIterator < Item = (A , B) > , { execute_into (& mut self . 0 , & mut self . 1 , pi . into_par_iter () , Unzip) ; } }
};
}
