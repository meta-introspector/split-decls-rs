// Generated macro for impl_62 (impl)
macro_rules! Depcrate_slice_iterimpl_62 {
() => {
// Module: crate::slice::iter
// Provides: {"impl_62"}
// Dependencies: {}
impl < 'a , T > Index < usize > for SliceCopyIter < 'a , T > where T : Copy , { type Output = T ; fn index (& self , i : usize) -> & T { assert ! (i < self . len ()) ; unsafe { & * self . ptr . offset (i as isize) } } }
};
}
