// Generated macro for impl_194 (impl)
macro_rules! Depcrate_ord_mapimpl_194 {
() => {
// Module: crate::ord::map
// Provides: {"impl_194"}
// Dependencies: {}
impl < 'a , BK , K , V > Index < & 'a BK > for OrdMap < K , V > where BK : Ord + ? Sized , K : Ord + Borrow < BK > , { type Output = V ; fn index (& self , key : & BK) -> & Self :: Output { match self . root . lookup (key) { None => panic ! ("OrdMap::index: invalid key") , Some (& (_ , ref value)) => value , } } }
};
}
