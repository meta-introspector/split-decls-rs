// Generated macro for parallel_iterator_methods (macro)
macro_rules! Depcrate_macrosparallel_iterator_methods {
() => {
// Module: crate::macros
// Provides: {"parallel_iterator_methods"}
// Dependencies: {}
# [cfg (feature = "rayon")] macro_rules ! parallel_iterator_methods { ($ map_elt : expr) => { fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item >, { self . entries . into_par_iter () . map ($ map_elt) . drive_unindexed (consumer) } fn opt_len (& self) -> Option < usize > { Some (self . entries . len ()) } } ; }
};
}
