// Generated macro for impl_1060 (impl)
macro_rules! Depcrate_collections_vec_dequeimpl_1060 {
() => {
// Module: crate::collections::vec_deque
// Provides: {"impl_1060"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T , A : Allocator > IndexMut < usize > for VecDeque < T , A > { # [inline] fn index_mut (& mut self , index : usize) -> & mut T { self . get_mut (index) . expect ("Out of bounds access") } }
};
}
