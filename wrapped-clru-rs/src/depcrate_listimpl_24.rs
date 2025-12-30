// Generated macro for impl_24 (impl)
macro_rules! Depcrate_listimpl_24 {
() => {
// Module: crate::list
// Provides: {"impl_24"}
// Dependencies: {}
impl < T > DoubleEndedIterator for FixedSizeListIterMut < '_ , T > { # [allow (unsafe_code)] fn next_back (& mut self) -> Option < Self :: Item > { if self . len > 0 { let back = self . back ; let node_ref = unsafe { let ptr = NonNull :: new_unchecked (self . ptr . as_ptr () . add (back)) . as_ptr () ; & mut * ptr } ; let node = node_ref . as_mut () . unwrap () ; self . back = node . prev ; self . len -= 1 ; Some ((back , & mut node . data)) } else { None } } }
};
}
