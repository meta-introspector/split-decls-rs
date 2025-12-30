// Generated macro for impl_416 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_416 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_416"}
// Dependencies: {}
impl < K , V > BTreeMap < K , V > { # [doc = " Makes a new, empty `BTreeMap`."] # [doc = ""] # [doc = " Does not allocate anything on its own."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::BTreeMap;"] # [doc = ""] # [doc = " let mut map = BTreeMap::new();"] # [doc = ""] # [doc = " // entries can now be inserted into the empty map"] # [doc = " map.insert(1, \"a\");"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] # [rustc_const_stable (feature = "const_btree_new" , since = "1.66.0")] # [inline] # [must_use] pub const fn new () -> BTreeMap < K , V > { BTreeMap { root : None , length : 0 , alloc : ManuallyDrop :: new (Global) , _marker : PhantomData } } }
};
}
