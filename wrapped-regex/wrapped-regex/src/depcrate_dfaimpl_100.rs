// Generated macro for impl_100 (impl)
macro_rules! Depcrate_dfaimpl_100 {
() => {
// Module: crate::dfa
// Provides: {"impl_100"}
// Dependencies: {}
impl Cache { # [doc = " Create new empty cache for the DFA engine."] pub fn new (prog : & Program) -> Self { let num_byte_classes = (prog . byte_classes [255] as usize + 1) + 1 ; let starts = vec ! [STATE_UNKNOWN ; 256] ; let mut cache = Cache { inner : CacheInner { compiled : HashMap :: new () , trans : Transitions :: new (num_byte_classes) , states : vec ! [] , start_states : starts , stack : vec ! [] , flush_count : 0 , size : 0 , } , qcur : SparseSet :: new (prog . insts . len ()) , qnext : SparseSet :: new (prog . insts . len ()) , } ; cache . inner . reset_size () ; cache } }
};
}
