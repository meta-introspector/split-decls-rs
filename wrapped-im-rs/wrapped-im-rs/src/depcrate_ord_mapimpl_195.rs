// Generated macro for impl_195 (impl)
macro_rules! Depcrate_ord_mapimpl_195 {
() => {
// Module: crate::ord::map
// Provides: {"impl_195"}
// Dependencies: {}
impl < 'a , BK , K , V > IndexMut < & 'a BK > for OrdMap < K , V > where BK : Ord + ? Sized , K : Ord + Clone + Borrow < BK > , V : Clone , { fn index_mut (& mut self , key : & BK) -> & mut Self :: Output { let root = PoolRef :: make_mut (& self . pool . 0 , & mut self . root) ; match root . lookup_mut (& self . pool . 0 , key) { None => panic ! ("OrdMap::index: invalid key") , Some (& mut (_ , ref mut value)) => value , } } }
};
}
