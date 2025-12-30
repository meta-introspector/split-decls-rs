// Generated macro for impl_864 (impl)
macro_rules! Depcrate_collections_linked_listimpl_864 {
() => {
// Module: crate::collections::linked_list
// Provides: {"impl_864"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : PartialEq , A : Allocator > PartialEq for LinkedList < T , A > { fn eq (& self , other : & Self) -> bool { self . len () == other . len () && self . iter () . eq (other) } fn ne (& self , other : & Self) -> bool { self . len () != other . len () || self . iter () . ne (other) } }
};
}
