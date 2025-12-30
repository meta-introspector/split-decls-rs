// Generated macro for impl_48 (impl)
macro_rules! Depcrate_baseimpl_48 {
() => {
// Module: crate::base
// Provides: {"impl_48"}
// Dependencies: {}
impl < 'a , K , V > RefEntry < 'a , K , V > where K : Ord , { # [doc = " Moves to the next entry in the skip list."] pub fn move_next (& mut self , guard : & Guard) -> bool { match self . next (guard) { None => false , Some (e) => { mem :: replace (self , e) . release (guard) ; true } } } # [doc = " Returns the next entry in the skip list."] pub fn next (& self , guard : & Guard) -> Option < RefEntry < 'a , K , V > > { self . parent . check_guard (guard) ; unsafe { let mut n = self . node ; loop { n = self . parent . next_node (& n . tower , Bound :: Excluded (& n . key) , guard) ? ; if let Some (e) = RefEntry :: try_acquire (self . parent , n) { return Some (e) ; } } } } # [doc = " Moves to the previous entry in the skip list."] pub fn move_prev (& mut self , guard : & Guard) -> bool { match self . prev (guard) { None => false , Some (e) => { mem :: replace (self , e) . release (guard) ; true } } } # [doc = " Returns the previous entry in the skip list."] pub fn prev (& self , guard : & Guard) -> Option < RefEntry < 'a , K , V > > { self . parent . check_guard (guard) ; unsafe { let mut n = self . node ; loop { n = self . parent . search_bound (Bound :: Excluded (& n . key) , true , guard) ? ; if let Some (e) = RefEntry :: try_acquire (self . parent , n) { return Some (e) ; } } } } }
};
}
