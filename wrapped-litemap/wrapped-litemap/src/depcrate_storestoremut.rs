// Generated macro for StoreMut (trait)
macro_rules! Depcrate_storeStoreMut {
() => {
// Module: crate::store
// Provides: {"StoreMut"}
// Dependencies: {}
pub trait StoreMut < K , V > : Store < K , V > { # [doc = " Creates a new store with the specified capacity hint."] # [doc = ""] # [doc = " Implementations may ignore the argument if they do not support pre-allocating capacity."] fn lm_with_capacity (capacity : usize) -> Self ; # [doc = " Reserves additional capacity in the store."] # [doc = ""] # [doc = " Implementations may ignore the argument if they do not support pre-allocating capacity."] fn lm_reserve (& mut self , additional : usize) ; # [doc = " Gets a key/value pair at the specified index, with a mutable value."] fn lm_get_mut (& mut self , index : usize) -> Option < (& K , & mut V) > ; # [doc = " Pushes one additional item onto the store."] fn lm_push (& mut self , key : K , value : V) ; # [doc = " Inserts an item at a specific index in the store."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `index` is greater than the length."] fn lm_insert (& mut self , index : usize , key : K , value : V) ; # [doc = " Removes an item at a specific index in the store."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `index` is greater than the length."] fn lm_remove (& mut self , index : usize) -> (K , V) ; # [doc = " Removes all items from the store."] fn lm_clear (& mut self) ; }
};
}
