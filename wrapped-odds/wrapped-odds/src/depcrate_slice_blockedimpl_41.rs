// Generated macro for impl_41 (impl)
macro_rules! Depcrate_slice_blockedimpl_41 {
() => {
// Module: crate::slice::blocked
// Provides: {"impl_41"}
// Dependencies: {}
impl < 'a , B , T > Iterator for BlockedIter < 'a , B , T > where B : Block < Item = T > , { type Item = & 'a B ; fn next (& mut self) -> Option < Self :: Item > { if ptrdistance (self . ptr , self . end) >= B :: capacity () { unsafe { let elt = Some (& * (self . ptr as * const B)) ; self . ptr = self . ptr . offset (B :: capacity () as isize) ; elt } } else { None } } fn size_hint (& self) -> (usize , Option < usize >) { let len = ptrdistance (self . ptr , self . end) / B :: capacity () ; (len , Some (len)) } }
};
}
