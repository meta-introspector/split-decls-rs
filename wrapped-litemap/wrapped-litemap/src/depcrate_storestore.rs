// Generated macro for Store (trait)
macro_rules! Depcrate_storeStore {
() => {
// Module: crate::store
// Provides: {"Store"}
// Dependencies: {}
# [doc = " Trait to enable pluggable backends for LiteMap."] # [doc = ""] # [doc = " Some methods have default implementations provided for convenience; however, it is generally"] # [doc = " better to implement all methods that your data store supports."] pub trait Store < K : ? Sized , V : ? Sized > : Sized { # [doc = " Returns the number of elements in the store."] fn lm_len (& self) -> usize ; # [doc = " Returns whether the store is empty (contains 0 elements)."] fn lm_is_empty (& self) -> bool { self . lm_len () == 0 } # [doc = " Gets a key/value pair at the specified index."] fn lm_get (& self , index : usize) -> Option < (& K , & V) > ; # [doc = " Gets the last element in the store, or `None` if the store is empty."] fn lm_last (& self) -> Option < (& K , & V) > { let len = self . lm_len () ; if len == 0 { None } else { self . lm_get (len - 1) } } # [doc = " Searches the store for a particular element with a comparator function."] # [doc = ""] # [doc = " See the binary search implementation on `slice` for more information."] fn lm_binary_search_by < F > (& self , cmp : F) -> Result < usize , usize > where F : FnMut (& K) -> Ordering ; }
};
}
