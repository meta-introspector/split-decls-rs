// Generated macro for impl_157 (impl)
macro_rules! Depcrate_strideimpl_157 {
() => {
// Module: crate::stride
// Provides: {"impl_157"}
// Dependencies: {}
impl < 'a , A > IndexMut < usize > for StrideMut < 'a , A > { # [doc = " Return a mutable reference to the element at a given index."] # [doc = ""] # [doc = " **Panics** if the index is out of bounds."] fn index_mut < 'b > (& 'b mut self , i : usize) -> & 'b mut A { assert ! (i < self . len ()) ; unsafe { let ptr = self . begin . offset (self . offset + self . stride * (i as isize)) ; & mut * ptr } } }
};
}
