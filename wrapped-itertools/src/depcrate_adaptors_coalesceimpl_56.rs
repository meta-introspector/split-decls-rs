// Generated macro for impl_56 (impl)
macro_rules! Depcrate_adaptors_coalesceimpl_56 {
() => {
// Module: crate::adaptors::coalesce
// Provides: {"impl_56"}
// Dependencies: {}
impl < F , Item , T > CoalescePredicate < Item , T > for F where F : FnMut (T , Item) -> Result < T , (T , T) > , { fn coalesce_pair (& mut self , t : T , item : Item) -> Result < T , (T , T) > { self (t , item) } }
};
}
