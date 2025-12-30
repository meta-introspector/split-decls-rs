// Generated macro for impl_490 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_490 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_490"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < K , Q : ? Sized , V , A : Allocator + Clone > Index < & Q > for BTreeMap < K , V , A > where K : Borrow < Q > + Ord , Q : Ord , { type Output = V ; # [doc = " Returns a reference to the value corresponding to the supplied key."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the key is not present in the `BTreeMap`."] # [inline] fn index (& self , key : & Q) -> & V { self . get (key) . expect ("no entry found for key") } }
};
}
