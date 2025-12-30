// Generated macro for impl_75 (impl)
macro_rules! Depcrate_atomicimpl_75 {
() => {
// Module: crate::atomic
// Provides: {"impl_75"}
// Dependencies: {}
impl < 'g , T : ? Sized + Pointable > PartialOrd < Shared < 'g , T > > for Shared < 'g , T > { fn partial_cmp (& self , other : & Self) -> Option < cmp :: Ordering > { Some (self . data . cmp (& other . data)) } }
};
}
