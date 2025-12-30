// Generated macro for impl_1175 (impl)
macro_rules! Depcrate_base_par_iterimpl_1175 {
() => {
// Module: crate::base::par_iter
// Provides: {"impl_1175"}
// Dependencies: {}
# [cfg_attr (doc_cfg , doc (cfg (feature = "rayon")))] # [doc = " *only available if compiled with the feature `rayon`*"] impl < 'a , T , R , Cols , S > ParColumnIterMut < 'a , T , R , Cols , S > where R : Dim , Cols : Dim , S : RawStorage < T , R , Cols > + RawStorageMut < T , R , Cols > , { # [doc = " create a new parallel iterator for the given matrix."] fn new (mat : & 'a mut Matrix < T , R , Cols , S >) -> Self { Self { mat } } }
};
}
