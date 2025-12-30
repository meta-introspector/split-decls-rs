// Generated macro for BTreeValue (trait)
macro_rules! Depcrate_nodes_btreeBTreeValue {
() => {
// Module: crate::nodes::btree
// Provides: {"BTreeValue"}
// Dependencies: {}
pub trait BTreeValue { type Key ; fn ptr_eq (& self , other : & Self) -> bool ; fn search_key < BK > (slice : & [Self] , key : & BK) -> Result < usize , usize > where BK : Ord + ? Sized , Self : Sized , Self :: Key : Borrow < BK > ; fn search_value (slice : & [Self] , value : & Self) -> Result < usize , usize > where Self : Sized ; fn cmp_keys < BK > (& self , other : & BK) -> Ordering where BK : Ord + ? Sized , Self :: Key : Borrow < BK > ; fn cmp_values (& self , other : & Self) -> Ordering ; }
};
}
