// Generated macro for impl_31 (impl)
macro_rules! Depcrate_baseimpl_31 {
() => {
// Module: crate::base
// Provides: {"impl_31"}
// Dependencies: {}
impl < K , V > SkipList < K , V > { # [doc = " Returns a new, empty skip list."] pub fn new (collector : Collector) -> Self { Self { head : Head :: new () , collector , hot_data : CachePadded :: new (HotData { seed : AtomicUsize :: new (1) , len : AtomicUsize :: new (0) , max_height : AtomicUsize :: new (1) , }) , } } # [doc = " Returns `true` if the skip list is empty."] pub fn is_empty (& self) -> bool { self . len () == 0 } # [doc = " Returns the number of entries in the skip list."] # [doc = ""] # [doc = " If the skip list is being concurrently modified, consider the returned number just an"] # [doc = " approximation without any guarantees."] pub fn len (& self) -> usize { let len = self . hot_data . len . load (Ordering :: Relaxed) ; if len > isize :: MAX as usize { 0 } else { len } } # [doc = " Ensures that all `Guard`s used with the skip list come from the same"] # [doc = " `Collector`."] fn check_guard (& self , guard : & Guard) { if let Some (c) = guard . collector () { assert ! (c == & self . collector) ; } } }
};
}
