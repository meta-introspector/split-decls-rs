// Generated macro for impl_58 (impl)
macro_rules! Depcrate_parallel_in_orderimpl_58 {
() => {
// Module: crate::parallel::in_order
// Provides: {"impl_58"}
// Dependencies: {}
impl < T , E , I > From < I > for InOrderIter < T , I > where I : Iterator < Item = Result < (SequenceId , T) , E > > , { fn from (iter : I) -> Self { InOrderIter { inner : iter , store : Default :: default () , next_chunk : 0 , is_done : false , } } }
};
}
