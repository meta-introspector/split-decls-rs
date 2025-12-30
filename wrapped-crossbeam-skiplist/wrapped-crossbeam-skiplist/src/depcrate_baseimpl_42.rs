// Generated macro for impl_42 (impl)
macro_rules! Depcrate_baseimpl_42 {
() => {
// Module: crate::base
// Provides: {"impl_42"}
// Dependencies: {}
impl < 'a : 'g , 'g , K , V > Entry < 'a , 'g , K , V > where K : Ord , { # [doc = " Moves to the next entry in the skip list."] pub fn move_next (& mut self) -> bool { match self . next () { None => false , Some (n) => { * self = n ; true } } } # [doc = " Returns the next entry in the skip list."] pub fn next (& self) -> Option < Entry < 'a , 'g , K , V > > { let n = self . parent . next_node (& self . node . tower , Bound :: Excluded (& self . node . key) , self . guard ,) ? ; Some (Entry { parent : self . parent , node : n , guard : self . guard , }) } # [doc = " Moves to the previous entry in the skip list."] pub fn move_prev (& mut self) -> bool { match self . prev () { None => false , Some (n) => { * self = n ; true } } } # [doc = " Returns the previous entry in the skip list."] pub fn prev (& self) -> Option < Entry < 'a , 'g , K , V > > { let n = self . parent . search_bound (Bound :: Excluded (& self . node . key) , true , self . guard) ? ; Some (Entry { parent : self . parent , node : n , guard : self . guard , }) } }
};
}
