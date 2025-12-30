// Generated macro for AtomicElisionExt (trait)
macro_rules! Depcrate_elisionAtomicElisionExt {
() => {
// Module: crate::elision
// Provides: {"AtomicElisionExt"}
// Dependencies: {}
pub trait AtomicElisionExt { type IntType ; fn elision_compare_exchange_acquire (& self , current : Self :: IntType , new : Self :: IntType ,) -> Result < Self :: IntType , Self :: IntType > ; fn elision_fetch_sub_release (& self , val : Self :: IntType) -> Self :: IntType ; }
};
}
