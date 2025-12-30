// Generated macro for impl_1176 (impl)
macro_rules! Depcrate_base_par_iterimpl_1176 {
() => {
// Module: crate::base::par_iter
// Provides: {"impl_1176"}
// Dependencies: {}
# [cfg_attr (doc_cfg , doc (cfg (feature = "rayon")))] # [doc = " *Only available if compiled with the feature `rayon`*"] impl < 'a , T , R , Cols , S > ParallelIterator for ParColumnIterMut < 'a , T , R , Cols , S > where R : Dim , Cols : Dim , S : RawStorage < T , R , Cols > + RawStorageMut < T , R , Cols > , T : Send + Sync + Scalar , S : Send + Sync , { type Item = MatrixViewMut < 'a , T , R , U1 , S :: RStride , S :: CStride > ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : rayon :: iter :: plumbing :: UnindexedConsumer < Self :: Item > , { bridge (self , consumer) } fn opt_len (& self) -> Option < usize > { Some (self . mat . ncols ()) } }
};
}
