// Generated macro for impl_24 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_24 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_24"}
// Dependencies: {}
impl < 'a , K , V , S , Q > Index < & 'a Q > for LinkedHashMap < K , V , S > where K : Hash + Eq + Borrow < Q > , S : BuildHasher , Q : Eq + Hash + ? Sized , { type Output = V ; # [inline] fn index (& self , index : & 'a Q) -> & V { self . get (index) . expect ("no entry found for key") } }
};
}
