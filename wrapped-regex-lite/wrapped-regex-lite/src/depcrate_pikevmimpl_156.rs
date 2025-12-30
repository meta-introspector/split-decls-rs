// Generated macro for impl_156 (impl)
macro_rules! Depcrate_pikevmimpl_156 {
() => {
// Module: crate::pikevm
// Provides: {"impl_156"}
// Dependencies: {}
impl ActiveStates { # [doc = " Create a new set of active states for the given PikeVM. The active"] # [doc = " states returned may only be used with the given PikeVM. (Use 'reset'"] # [doc = " to re-purpose the allocation for a different PikeVM.)"] fn new (re : & PikeVM) -> ActiveStates { let mut active = ActiveStates { set : SparseSet :: new (0) , slot_table : SlotTable :: new () , } ; active . reset (re) ; active } # [doc = " Reset this set of active states such that it can be used with the given"] # [doc = " PikeVM (and only that PikeVM)."] fn reset (& mut self , re : & PikeVM) { self . set . resize (re . nfa () . len ()) ; self . slot_table . reset (re) ; } # [doc = " Setup this set of active states for a new search. The given slot"] # [doc = " length should be the number of slots in a caller provided 'Captures'"] # [doc = " (and may be zero)."] fn setup_search (& mut self , captures_slot_len : usize) { self . set . clear () ; self . slot_table . setup_search (captures_slot_len) ; } }
};
}
