// Generated macro for impl_533 (impl)
macro_rules! Depcrate_vectorimpl_533 {
() => {
// Module: crate::vector
// Provides: {"impl_533"}
// Dependencies: {}
impl < A : Clone > IndexMut < usize > for Vector < A > { # [doc = " Get a mutable reference to the value at index `index` in the"] # [doc = " vector."] # [doc = ""] # [doc = " Time: O(log n)"] fn index_mut (& mut self , index : usize) -> & mut Self :: Output { match self . get_mut (index) { Some (value) => value , None => panic ! ("Vector::index_mut: index out of bounds") , } } }
};
}
