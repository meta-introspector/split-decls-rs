// Generated macro for impl_336 (impl)
macro_rules! Depcrate_hash_mapimpl_336 {
() => {
// Module: crate::hash::map
// Provides: {"impl_336"}
// Dependencies: {}
impl < 'a , BK , K , V , S > IndexMut < & 'a BK > for HashMap < K , V , S > where BK : Hash + Eq + ? Sized , K : Hash + Eq + Clone + Borrow < BK > , V : Clone , S : BuildHasher , { fn index_mut (& mut self , key : & BK) -> & mut Self :: Output { let root = PoolRef :: make_mut (& self . pool . 0 , & mut self . root) ; match root . get_mut (& self . pool . 0 , hash_key (& * self . hasher , key) , 0 , key) { None => panic ! ("HashMap::index_mut: invalid key") , Some (& mut (_ , ref mut value)) => value , } } }
};
}
