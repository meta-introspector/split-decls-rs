// Generated macro for impl_81 (impl)
macro_rules! Depcrate_slice_revimpl_81 {
() => {
// Module: crate::slice::rev
// Provides: {"impl_81"}
// Dependencies: {}
impl < T > IndexMut < usize > for RevSlice < T > { fn index_mut (& mut self , i : usize) -> & mut T { let len = self . len () ; if let Some (x) = self . get_mut (i) { return x ; } else { panic ! ("Index {} is out of bounds for RevSlice of length {}" , i , len) ; } } }
};
}
