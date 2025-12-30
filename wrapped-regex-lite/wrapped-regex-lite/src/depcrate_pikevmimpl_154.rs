// Generated macro for impl_154 (impl)
macro_rules! Depcrate_pikevmimpl_154 {
() => {
// Module: crate::pikevm
// Provides: {"impl_154"}
// Dependencies: {}
impl Cache { # [doc = " Create a new `PikeVM` cache."] # [doc = ""] # [doc = " A potentially more convenient routine to create a cache is"] # [doc = " `PikeVM::create_cache`, as it does not require also importing the"] # [doc = " `Cache` type."] # [doc = ""] # [doc = " If you want to reuse the returned `Cache` with some other `PikeVM`,"] # [doc = " then you must call `Cache::reset` with the desired `PikeVM`."] pub (crate) fn new (re : & PikeVM) -> Cache { Cache { stack : vec ! [] , curr : ActiveStates :: new (re) , next : ActiveStates :: new (re) , } } # [doc = " Clears this cache. This should be called at the start of every search"] # [doc = " to ensure we start with a clean slate."] # [doc = ""] # [doc = " This also sets the length of the capturing groups used in the current"] # [doc = " search. This permits an optimization where by 'SlotTable::for_state'"] # [doc = " only returns the number of slots equivalent to the number of slots"] # [doc = " given in the 'Captures' value. This may be less than the total number"] # [doc = " of possible slots, e.g., when one only wants to track overall match"] # [doc = " offsets. This in turn permits less copying of capturing group spans"] # [doc = " in the PikeVM."] fn setup_search (& mut self , captures_slot_len : usize) { self . stack . clear () ; self . curr . setup_search (captures_slot_len) ; self . next . setup_search (captures_slot_len) ; } }
};
}
