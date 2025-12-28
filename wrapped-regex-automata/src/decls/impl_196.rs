macro_rules! deps {
    () => {
        Remapper!();
        Remappable!();
        IndexMapper!();
        StateID!();
    };
}

macro_rules! impl_196 {
    () => {
        deps!();
        impl Remapper { # [doc = " Create a new remapper from the given remappable implementation. The"] # [doc = " remapper can then be used to swap states. The remappable value given"] # [doc = " here must the same one given to `swap` and `remap`."] pub (super) fn new (r : & impl Remappable) -> Remapper { let idxmap = IndexMapper { stride2 : r . stride2 () } ; let map = (0 .. r . state_len ()) . map (| i | idxmap . to_state_id (i)) . collect () ; Remapper { map , idxmap } } # [doc = " Swap two states. Once this is called, callers must follow through to"] # [doc = " call `remap`, or else it's possible for the underlying remappable"] # [doc = " value to be in a corrupt state."] pub (super) fn swap (& mut self , r : & mut impl Remappable , id1 : StateID , id2 : StateID ,) { if id1 == id2 { return ; } r . swap_states (id1 , id2) ; self . map . swap (self . idxmap . to_index (id1) , self . idxmap . to_index (id2)) ; } # [doc = " Complete the remapping process by rewriting all state IDs in the"] # [doc = " remappable value according to the swaps performed."] pub (super) fn remap (mut self , r : & mut impl Remappable) { let oldmap = self . map . clone () ; for i in 0 .. r . state_len () { let cur_id = self . idxmap . to_state_id (i) ; let mut new_id = oldmap [i] ; if cur_id == new_id { continue ; } loop { let id = oldmap [self . idxmap . to_index (new_id)] ; if cur_id == id { self . map [i] = new_id ; break ; } new_id = id ; } } r . remap (| next | self . map [self . idxmap . to_index (next)]) ; } }
    };
}

impl_196!();