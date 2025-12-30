// Generated macro for impl_274 (impl)
macro_rules! Depcrate_const_eval_machineimpl_274 {
() => {
// Module: crate::const_eval::machine
// Provides: {"impl_274"}
// Dependencies: {}
impl < K : Hash + Eq , V > interpret :: AllocMap < K , V > for FxIndexMap < K , V > { # [inline (always)] fn contains_key < Q : ? Sized + Hash + Eq > (& mut self , k : & Q) -> bool where K : Borrow < Q > , { FxIndexMap :: contains_key (self , k) } # [inline (always)] fn contains_key_ref < Q : ? Sized + Hash + Eq > (& self , k : & Q) -> bool where K : Borrow < Q > , { FxIndexMap :: contains_key (self , k) } # [inline (always)] fn insert (& mut self , k : K , v : V) -> Option < V > { FxIndexMap :: insert (self , k , v) } # [inline (always)] fn remove < Q : ? Sized + Hash + Eq > (& mut self , k : & Q) -> Option < V > where K : Borrow < Q > , { FxIndexMap :: swap_remove (self , k) } # [inline (always)] fn filter_map_collect < T > (& self , mut f : impl FnMut (& K , & V) -> Option < T >) -> Vec < T > { self . iter () . filter_map (move | (k , v) | f (k , v)) . collect () } # [inline (always)] fn get_or < E > (& self , k : K , vacant : impl FnOnce () -> Result < V , E >) -> Result < & V , E > { match self . get (& k) { Some (v) => Ok (v) , None => { vacant () ? ; bug ! ("The CTFE machine shouldn't ever need to extend the alloc_map when reading") } } } # [inline (always)] fn get_mut_or < E > (& mut self , k : K , vacant : impl FnOnce () -> Result < V , E >) -> Result < & mut V , E > { match self . entry (k) { IndexEntry :: Occupied (e) => Ok (e . into_mut ()) , IndexEntry :: Vacant (e) => { let v = vacant () ? ; Ok (e . insert (v)) } } } }
};
}
