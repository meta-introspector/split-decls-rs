// Generated macro for impl_18 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_18 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_18"}
// Dependencies: {}
impl < K : Hash + Eq + PartialOrd , V : PartialOrd , S : BuildHasher > PartialOrd for LinkedHashMap < K , V , S > { # [inline] fn partial_cmp (& self , other : & Self) -> Option < Ordering > { self . iter () . partial_cmp (other) } # [inline] fn lt (& self , other : & Self) -> bool { self . iter () . lt (other) } # [inline] fn le (& self , other : & Self) -> bool { self . iter () . le (other) } # [inline] fn ge (& self , other : & Self) -> bool { self . iter () . ge (other) } # [inline] fn gt (& self , other : & Self) -> bool { self . iter () . gt (other) } }
};
}
