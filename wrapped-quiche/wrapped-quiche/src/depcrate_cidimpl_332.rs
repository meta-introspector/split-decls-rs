// Generated macro for impl_332 (impl)
macro_rules! Depcrate_cidimpl_332 {
() => {
// Module: crate::cid
// Provides: {"impl_332"}
// Dependencies: {}
impl BoundedConnectionIdSeqSet { # [doc = " Creates a set bounded by `capacity`."] fn new (capacity : usize) -> Self { Self { inner : HashSet :: new () , capacity , } } fn insert (& mut self , e : u64) -> Result < bool > { if self . inner . len () >= self . capacity { return Err (Error :: IdLimit) ; } Ok (self . inner . insert (e)) } fn remove (& mut self , e : & u64) -> bool { self . inner . remove (e) } fn is_empty (& self) -> bool { self . inner . is_empty () } }
};
}
