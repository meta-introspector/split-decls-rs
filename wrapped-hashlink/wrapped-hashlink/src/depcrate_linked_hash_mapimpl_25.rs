// Generated macro for impl_25 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_25 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_25"}
// Dependencies: {}
impl < 'a , K , V , S , Q > IndexMut < & 'a Q > for LinkedHashMap < K , V , S > where K : Hash + Eq + Borrow < Q > , S : BuildHasher , Q : Eq + Hash + ? Sized , { # [inline] fn index_mut (& mut self , index : & 'a Q) -> & mut V { self . get_mut (index) . expect ("no entry found for key") } }
};
}
