// Generated macro for indexed_parallel_iterator_methods (macro)
macro_rules! Depcrate_macrosindexed_parallel_iterator_methods {
() => {
// Module: crate::macros
// Provides: {"indexed_parallel_iterator_methods"}
// Dependencies: {}
# [cfg (feature = "rayon")] macro_rules ! indexed_parallel_iterator_methods { ($ map_elt : expr) => { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item >, { self . entries . into_par_iter () . map ($ map_elt) . drive (consumer) } fn len (& self) -> usize { self . entries . len () } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item >, { self . entries . into_par_iter () . map ($ map_elt) . with_producer (callback) } } ; }
};
}
