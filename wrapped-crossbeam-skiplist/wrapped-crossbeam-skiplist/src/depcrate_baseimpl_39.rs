// Generated macro for impl_39 (impl)
macro_rules! Depcrate_baseimpl_39 {
() => {
// Module: crate::base
// Provides: {"impl_39"}
// Dependencies: {}
impl < K , V > Entry < '_ , '_ , K , V > where K : Ord + Send + 'static , V : Send + 'static , { # [doc = " Removes the entry from the skip list."] # [doc = ""] # [doc = " Returns `true` if this call removed the entry and `false` if it was already removed."] pub fn remove (& self) -> bool { if self . node . mark_tower () { self . parent . hot_data . len . fetch_sub (1 , Ordering :: Relaxed) ; self . parent . search_bound (Bound :: Included (& self . node . key) , false , self . guard) ; true } else { false } } }
};
}
