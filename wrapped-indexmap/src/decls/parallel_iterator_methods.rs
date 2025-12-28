macro_rules! parallel_iterator_methods {
    () => {
        # [cfg (feature = "rayon")] macro_rules ! parallel_iterator_methods { ($ map_elt : expr) => { fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item >, { self . entries . into_par_iter () . map ($ map_elt) . drive_unindexed (consumer) } fn opt_len (& self) -> Option < usize > { Some (self . entries . len ()) } } ; }
    };
}

parallel_iterator_methods!();