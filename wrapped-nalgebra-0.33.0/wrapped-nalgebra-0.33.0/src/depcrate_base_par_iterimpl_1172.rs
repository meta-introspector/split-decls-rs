// Generated macro for impl_1172 (impl)
macro_rules! Depcrate_base_par_iterimpl_1172 {
() => {
// Module: crate::base::par_iter
// Provides: {"impl_1172"}
// Dependencies: {}
# [cfg_attr (doc_cfg , doc (cfg (feature = "rayon")))] impl < 'a , T , R : Dim , Cols : Dim , S : RawStorage < T , R , Cols > > ParallelIterator for ParColumnIter < 'a , T , R , Cols , S > where T : Sync + Send + Scalar , S : Sync , { type Item = MatrixView < 'a , T , R , U1 , S :: RStride , S :: CStride > ; fn drive_unindexed < Consumer > (self , consumer : Consumer) -> Consumer :: Result where Consumer : rayon :: iter :: plumbing :: UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . mat . ncols ()) } }
};
}
