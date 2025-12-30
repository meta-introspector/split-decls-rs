// Generated macro for impl_50 (impl)
macro_rules! Depcrate_baseimpl_50 {
() => {
// Module: crate::base
// Provides: {"impl_50"}
// Dependencies: {}
impl < 'a : 'g , 'g , K : 'a , V : 'a > Iterator for Iter < 'a , 'g , K , V > where K : Ord , { type Item = Entry < 'a , 'g , K , V > ; fn next (& mut self) -> Option < Entry < 'a , 'g , K , V > > { self . head = match self . head { Some (n) => self . parent . next_node (& n . tower , Bound :: Excluded (& n . key) , self . guard) , None => self . parent . next_node (& self . parent . head , Bound :: Unbounded , self . guard) , } ; if let (Some (h) , Some (t)) = (self . head , self . tail) { if h . key >= t . key { self . head = None ; self . tail = None ; } } self . head . map (| n | Entry { parent : self . parent , node : n , guard : self . guard , }) } }
};
}
