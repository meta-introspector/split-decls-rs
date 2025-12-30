// Generated macro for impl_1173 (impl)
macro_rules! Depcrate_base_par_iterimpl_1173 {
() => {
// Module: crate::base::par_iter
// Provides: {"impl_1173"}
// Dependencies: {}
# [cfg_attr (doc_cfg , doc (cfg (feature = "rayon")))] # [doc = " *Only available if compiled with the feature `rayon`.*"] impl < 'a , T , R : Dim , Cols : Dim , S : RawStorage < T , R , Cols > > IndexedParallelIterator for ParColumnIter < 'a , T , R , Cols , S > where T : Send + Sync + Scalar , S : Sync , { fn len (& self) -> usize { self . mat . ncols () } fn drive < C : rayon :: iter :: plumbing :: Consumer < Self :: Item > > (self , consumer : C) -> C :: Result { bridge (self , consumer) } fn with_producer < CB : rayon :: iter :: plumbing :: ProducerCallback < Self :: Item > > (self , callback : CB ,) -> CB :: Output { let producer = ColumnProducer (ColumnIter :: new (self . mat)) ; callback . callback (producer) } }
};
}
