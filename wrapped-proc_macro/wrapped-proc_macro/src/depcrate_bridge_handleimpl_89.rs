// Generated macro for impl_89 (impl)
macro_rules! Depcrate_bridge_handleimpl_89 {
() => {
// Module: crate::bridge::handle
// Provides: {"impl_89"}
// Dependencies: {}
impl < T > OwnedStore < T > { pub (super) fn alloc (& mut self , x : T) -> Handle { let counter = self . counter . fetch_add (1 , Ordering :: Relaxed) ; let handle = Handle :: new (counter) . expect ("`proc_macro` handle counter overflowed") ; assert ! (self . data . insert (handle , x) . is_none ()) ; handle } pub (super) fn take (& mut self , h : Handle) -> T { self . data . remove (& h) . expect ("use-after-free in `proc_macro` handle") } }
};
}
