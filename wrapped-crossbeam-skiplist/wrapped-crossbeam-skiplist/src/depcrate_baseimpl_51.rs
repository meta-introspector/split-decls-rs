// Generated macro for impl_51 (impl)
macro_rules! Depcrate_baseimpl_51 {
() => {
// Module: crate::base
// Provides: {"impl_51"}
// Dependencies: {}
impl < 'a : 'g , 'g , K : 'a , V : 'a > DoubleEndedIterator for Iter < 'a , 'g , K , V > where K : Ord , { fn next_back (& mut self) -> Option < Entry < 'a , 'g , K , V > > { self . tail = match self . tail { Some (n) => self . parent . search_bound (Bound :: Excluded (& n . key) , true , self . guard) , None => self . parent . search_bound :: < K > (Bound :: Unbounded , true , self . guard) , } ; if let (Some (h) , Some (t)) = (self . head , self . tail) { if h . key >= t . key { self . head = None ; self . tail = None ; } } self . tail . map (| n | Entry { parent : self . parent , node : n , guard : self . guard , }) } }
};
}
