// Generated macro for impl_80 (impl)
macro_rules! Depcrate_slice_revimpl_80 {
() => {
// Module: crate::slice::rev
// Provides: {"impl_80"}
// Dependencies: {}
impl < T > Index < usize > for RevSlice < T > { type Output = T ; fn index (& self , i : usize) -> & T { if let Some (x) = self . get (i) { x } else { panic ! ("Index {} is out of bounds for RevSlice of length {}" , i , self . len ()) ; } } }
};
}
