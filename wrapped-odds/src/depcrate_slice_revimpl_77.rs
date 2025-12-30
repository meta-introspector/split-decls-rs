// Generated macro for impl_77 (impl)
macro_rules! Depcrate_slice_revimpl_77 {
() => {
// Module: crate::slice::rev
// Provides: {"impl_77"}
// Dependencies: {}
impl < 'a , T , Slice : ? Sized > From < & 'a Slice > for & 'a RevSlice < T > where Slice : AsRef < [T] > , { fn from (slc : & 'a Slice) -> Self { unsafe { transmute (slc . as_ref ()) } } }
};
}
