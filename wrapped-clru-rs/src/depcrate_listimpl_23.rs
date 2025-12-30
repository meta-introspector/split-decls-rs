// Generated macro for impl_23 (impl)
macro_rules! Depcrate_listimpl_23 {
() => {
// Module: crate::list
// Provides: {"impl_23"}
// Dependencies: {}
impl < 'a , T > Iterator for FixedSizeListIterMut < 'a , T > { type Item = (usize , & 'a mut T) ; # [allow (unsafe_code)] fn next (& mut self) -> Option < Self :: Item > { if self . len > 0 { let front = self . front ; let node_ref = unsafe { let ptr = NonNull :: new_unchecked (self . ptr . as_ptr () . add (front)) . as_ptr () ; & mut * ptr } ; let node = node_ref . as_mut () . unwrap () ; self . front = node . next ; self . len -= 1 ; Some ((front , & mut node . data)) } else { None } } fn size_hint (& self) -> (usize , Option < usize >) { (self . len , Some (self . len)) } }
};
}
