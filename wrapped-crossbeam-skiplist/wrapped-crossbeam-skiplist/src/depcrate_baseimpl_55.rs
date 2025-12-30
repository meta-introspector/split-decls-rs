// Generated macro for impl_55 (impl)
macro_rules! Depcrate_baseimpl_55 {
() => {
// Module: crate::base
// Provides: {"impl_55"}
// Dependencies: {}
impl < 'a , K : 'a , V : 'a > RefIter < 'a , K , V > where K : Ord , { # [doc = " Advances the iterator and returns the next value."] pub fn next (& mut self , guard : & Guard) -> Option < RefEntry < 'a , K , V > > { self . parent . check_guard (guard) ; let next_head = match & self . head { Some (e) => e . next (guard) , None => try_pin_loop (| | self . parent . front (guard)) , } ; match (& next_head , & self . tail) { (Some (ref next) , Some (t)) if next . key () >= t . key () => { unsafe { next . node . decrement (guard) ; } None } (Some (_) , _) => { if let Some (e) = mem :: replace (& mut self . head , next_head . clone ()) { unsafe { e . node . decrement (guard) ; } } next_head } (None , _) => None , } } # [doc = " Removes and returns an element from the end of the iterator."] pub fn next_back (& mut self , guard : & Guard) -> Option < RefEntry < 'a , K , V > > { self . parent . check_guard (guard) ; let next_tail = match & self . tail { Some (e) => e . prev (guard) , None => try_pin_loop (| | self . parent . back (guard)) , } ; match (& self . head , & next_tail) { (Some (h) , Some (next)) if h . key () >= next . key () => { unsafe { next . node . decrement (guard) ; } None } (_ , Some (_)) => { if let Some (e) = mem :: replace (& mut self . tail , next_tail . clone ()) { unsafe { e . node . decrement (guard) ; } } next_tail } (_ , None) => None , } } }
};
}
