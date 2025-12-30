// Generated macro for impl_58 (impl)
macro_rules! Depcrate_baseimpl_58 {
() => {
// Module: crate::base
// Provides: {"impl_58"}
// Dependencies: {}
impl < 'a : 'g , 'g , Q , R , K : 'a , V : 'a > Iterator for Range < 'a , 'g , Q , R , K , V > where K : Ord + Comparable < Q > , R : RangeBounds < Q > , Q : ? Sized , { type Item = Entry < 'a , 'g , K , V > ; fn next (& mut self) -> Option < Entry < 'a , 'g , K , V > > { self . head = match self . head { Some (n) => self . parent . next_node (& n . tower , Bound :: Excluded (& n . key) , self . guard) , None => self . parent . search_bound (self . range . start_bound () , false , self . guard) , } ; if let Some (h) = self . head { match self . tail { Some (t) => { let bound = Bound :: Excluded (& t . key) ; if ! below_upper_bound (& bound , & h . key) { self . head = None ; self . tail = None ; } } None => { let bound = self . range . end_bound () ; if ! below_upper_bound (& bound , & h . key) { self . head = None ; self . tail = None ; } } } ; } self . head . map (| n | Entry { parent : self . parent , node : n , guard : self . guard , }) } }
};
}
