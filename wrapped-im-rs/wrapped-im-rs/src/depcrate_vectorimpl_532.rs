// Generated macro for impl_532 (impl)
macro_rules! Depcrate_vectorimpl_532 {
() => {
// Module: crate::vector
// Provides: {"impl_532"}
// Dependencies: {}
impl < A : Clone > Index < usize > for Vector < A > { type Output = A ; # [doc = " Get a reference to the value at index `index` in the vector."] # [doc = ""] # [doc = " Time: O(log n)"] fn index (& self , index : usize) -> & Self :: Output { match self . get (index) { Some (value) => value , None => panic ! ("Vector::index: index out of bounds: {} < {}" , index , self . len ()) , } } }
};
}
