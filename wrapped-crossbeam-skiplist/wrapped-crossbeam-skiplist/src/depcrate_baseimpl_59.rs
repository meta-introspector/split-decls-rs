// Generated macro for impl_59 (impl)
macro_rules! Depcrate_baseimpl_59 {
() => {
// Module: crate::base
// Provides: {"impl_59"}
// Dependencies: {}
impl < 'a : 'g , 'g , Q , R , K : 'a , V : 'a > DoubleEndedIterator for Range < 'a , 'g , Q , R , K , V > where K : Ord + Comparable < Q > , R : RangeBounds < Q > , Q : ? Sized , { fn next_back (& mut self) -> Option < Entry < 'a , 'g , K , V > > { self . tail = match self . tail { Some (n) => self . parent . search_bound :: < K > (Bound :: Excluded (& n . key) , true , self . guard) , None => self . parent . search_bound (self . range . end_bound () , true , self . guard) , } ; if let Some (t) = self . tail { match self . head { Some (h) => { let bound = Bound :: Excluded (& h . key) ; if ! above_lower_bound (& bound , & t . key) { self . head = None ; self . tail = None ; } } None => { let bound = self . range . start_bound () ; if ! above_lower_bound (& bound , & t . key) { self . head = None ; self . tail = None ; } } } ; } self . tail . map (| n | Entry { parent : self . parent , node : n , guard : self . guard , }) } }
};
}
