// Generated macro for impl_335 (impl)
macro_rules! Depcrate_hash_mapimpl_335 {
() => {
// Module: crate::hash::map
// Provides: {"impl_335"}
// Dependencies: {}
impl < 'a , BK , K , V , S > Index < & 'a BK > for HashMap < K , V , S > where BK : Hash + Eq + ? Sized , K : Hash + Eq + Borrow < BK > , S : BuildHasher , { type Output = V ; fn index (& self , key : & BK) -> & Self :: Output { match self . root . get (hash_key (& * self . hasher , key) , 0 , key) { None => panic ! ("HashMap::index: invalid key") , Some (& (_ , ref value)) => value , } } }
};
}
