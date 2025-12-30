// Generated macro for impl_78 (impl)
macro_rules! Depcrate_slice_revimpl_78 {
() => {
// Module: crate::slice::rev
// Provides: {"impl_78"}
// Dependencies: {}
impl < 'a , T , Slice : ? Sized > From < & 'a mut Slice > for & 'a mut RevSlice < T > where Slice : AsMut < [T] > , { fn from (slc : & 'a mut Slice) -> Self { unsafe { transmute (slc . as_mut ()) } } }
};
}
