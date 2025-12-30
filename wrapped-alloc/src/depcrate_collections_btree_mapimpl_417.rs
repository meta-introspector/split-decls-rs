// Generated macro for impl_417 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_417 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_417"}
// Dependencies: {}
impl < K , V , A : Allocator + Clone > BTreeMap < K , V , A > { # [doc = " Clears the map, removing all elements."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::BTreeMap;"] # [doc = ""] # [doc = " let mut a = BTreeMap::new();"] # [doc = " a.insert(1, \"a\");"] # [doc = " a.clear();"] # [doc = " assert!(a.is_empty());"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] pub fn clear (& mut self) { drop (BTreeMap { root : mem :: replace (& mut self . root , None) , length : mem :: replace (& mut self . length , 0) , alloc : self . alloc . clone () , _marker : PhantomData , }) ; } # [doc = " Makes a new empty BTreeMap with a reasonable choice for B."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # #![feature(allocator_api)]"] # [doc = " # #![feature(btreemap_alloc)]"] # [doc = " use std::collections::BTreeMap;"] # [doc = " use std::alloc::Global;"] # [doc = ""] # [doc = " let mut map = BTreeMap::new_in(Global);"] # [doc = ""] # [doc = " // entries can now be inserted into the empty map"] # [doc = " map.insert(1, \"a\");"] # [doc = " ```"] # [unstable (feature = "btreemap_alloc" , issue = "32838")] pub const fn new_in (alloc : A) -> BTreeMap < K , V , A > { BTreeMap { root : None , length : 0 , alloc : ManuallyDrop :: new (alloc) , _marker : PhantomData } } }
};
}
