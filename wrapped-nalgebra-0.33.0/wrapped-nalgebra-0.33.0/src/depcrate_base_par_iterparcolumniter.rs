// Generated macro for ParColumnIter (struct)
macro_rules! Depcrate_base_par_iterParColumnIter {
() => {
// Module: crate::base::par_iter
// Provides: {"ParColumnIter"}
// Dependencies: {}
# [doc = " A rayon parallel iterator over the columns of a matrix. It is created"] # [doc = " using the [`par_column_iter`] method of [`Matrix`]."] # [doc = ""] # [doc = " *Only available if compiled with the feature `rayon`.*"] # [doc = " [`par_column_iter`]: crate::Matrix::par_column_iter"] # [doc = " [`Matrix`]: crate::Matrix"] # [cfg_attr (doc_cfg , doc (cfg (feature = "rayon")))] pub struct ParColumnIter < 'a , T , R : Dim , Cols : Dim , S : RawStorage < T , R , Cols > > { mat : & 'a Matrix < T , R , Cols , S > , }
};
}
