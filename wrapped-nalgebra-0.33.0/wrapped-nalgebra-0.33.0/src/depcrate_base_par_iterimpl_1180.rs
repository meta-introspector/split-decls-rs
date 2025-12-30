// Generated macro for impl_1180 (impl)
macro_rules! Depcrate_base_par_iterimpl_1180 {
() => {
// Module: crate::base::par_iter
// Provides: {"impl_1180"}
// Dependencies: {}
# [cfg_attr (doc_cfg , doc (cfg (feature = "rayon")))] # [doc = " *only available if compiled with the feature `rayon`*"] impl < 'a , T , R : Dim , Cols : Dim , S : RawStorage < T , R , Cols > > Producer for ColumnProducer < 'a , T , R , Cols , S > where T : Send + Sync + Scalar , S : Sync , { type Item = MatrixView < 'a , T , R , U1 , S :: RStride , S :: CStride > ; type IntoIter = ColumnIter < 'a , T , R , Cols , S > ; # [inline] fn into_iter (self) -> Self :: IntoIter { self . 0 } # [inline] fn split_at (self , index : usize) -> (Self , Self) { let (left_iter , right_iter) = self . 0 . split_at (index) ; (Self (left_iter) , Self (right_iter)) } }
};
}
