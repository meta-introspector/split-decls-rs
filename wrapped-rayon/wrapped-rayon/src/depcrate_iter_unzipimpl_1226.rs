// Generated macro for impl_1226 (impl)
macro_rules! Depcrate_iter_unzipimpl_1226 {
() => {
// Module: crate::iter::unzip
// Provides: {"impl_1226"}
// Dependencies: {}
impl < T , FromT > ParallelExtend < T > for Collector < FromT > where T : Send , FromT : Send + FromParallelIterator < T > , { fn par_extend < I > (& mut self , pi : I) where I : IntoParallelIterator < Item = T > , { debug_assert ! (self . result . is_none ()) ; self . result = Some (pi . into_par_iter () . collect ()) ; } }
};
}
