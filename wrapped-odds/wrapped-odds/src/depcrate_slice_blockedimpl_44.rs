// Generated macro for impl_44 (impl)
macro_rules! Depcrate_slice_blockedimpl_44 {
() => {
// Module: crate::slice::blocked
// Provides: {"impl_44"}
// Dependencies: {}
impl < 'a , B , T > Index < usize > for BlockedIter < 'a , B , T > where B : Block < Item = T > , { type Output = B ; fn index (& self , i : usize) -> & Self :: Output { assert ! (i < self . len ()) ; unsafe { & * (self . ptr . offset ((i * B :: capacity ()) as isize) as * const B) } } }
};
}
