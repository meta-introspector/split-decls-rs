// Generated macro for impl_156 (impl)
macro_rules! Depcrate_strideimpl_156 {
() => {
// Module: crate::stride
// Provides: {"impl_156"}
// Dependencies: {}
impl < 'a , A > StrideMut < 'a , A > { # [doc = " Return a mutable reference to the element of a stride at the"] # [doc = " given index, or None if the index is out of bounds."] pub fn get_mut < 'b > (& 'b mut self , i : usize) -> Option < & 'b mut A > { if i >= self . len () { None } else { unsafe { let ptr = self . begin . offset (self . offset + self . stride * (i as isize)) ; Some (& mut * ptr) } } } }
};
}
