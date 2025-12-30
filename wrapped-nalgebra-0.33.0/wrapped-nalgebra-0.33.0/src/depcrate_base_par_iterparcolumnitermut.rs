// Generated macro for ParColumnIterMut (struct)
macro_rules! Depcrate_base_par_iterParColumnIterMut {
() => {
// Module: crate::base::par_iter
// Provides: {"ParColumnIterMut"}
// Dependencies: {}
# [cfg_attr (doc_cfg , doc (cfg (feature = "rayon")))] # [doc = " A rayon parallel iterator through the mutable columns of a matrix."] # [doc = " *Only available if compiled with the feature `rayon`.*"] pub struct ParColumnIterMut < 'a , T , R : Dim , Cols : Dim , S : RawStorage < T , R , Cols > + RawStorageMut < T , R , Cols > , > { mat : & 'a mut Matrix < T , R , Cols , S > , }
};
}
