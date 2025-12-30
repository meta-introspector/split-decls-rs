// Generated macro for impl_1177 (impl)
macro_rules! Depcrate_base_par_iterimpl_1177 {
() => {
// Module: crate::base::par_iter
// Provides: {"impl_1177"}
// Dependencies: {}
# [cfg_attr (doc_cfg , doc (cfg (feature = "rayon")))] # [doc = " *Only available if compiled with the feature `rayon`*"] impl < 'a , T , R , Cols , S > IndexedParallelIterator for ParColumnIterMut < 'a , T , R , Cols , S > where R : Dim , Cols : Dim , S : RawStorage < T , R , Cols > + RawStorageMut < T , R , Cols > , T : Send + Sync + Scalar , S : Send + Sync , { fn drive < C : rayon :: iter :: plumbing :: Consumer < Self :: Item > > (self , consumer : C) -> C :: Result { bridge (self , consumer) } fn len (& self) -> usize { self . mat . ncols () } fn with_producer < CB : rayon :: iter :: plumbing :: ProducerCallback < Self :: Item > > (self , callback : CB ,) -> CB :: Output { let producer = ColumnProducerMut (ColumnIterMut :: new (self . mat)) ; callback . callback (producer) } }
};
}
