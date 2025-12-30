// Generated macro for impl_34 (impl)
macro_rules! Depcrate_core_index_pathimpl_34 {
() => {
// Module: crate::core::index_path
// Provides: {"impl_34"}
// Dependencies: {}
impl IndexPath { pub fn new (indices : Vec < usize >) -> IndexPath { IndexPath { indices } } pub fn adding (& self , index : usize) -> IndexPath { let mut indices = self . indices . clone () ; indices . push (index) ; IndexPath :: new (indices) } pub fn push (& mut self , index : usize) { self . indices . push (index) ; } pub fn increment_last (& mut self) { * self . indices . last_mut () . unwrap () += 1 ; } pub fn pop (& mut self) -> Option < usize > { self . indices . pop () } pub fn is_empty (& self) -> bool { self . indices . is_empty () } }
};
}
