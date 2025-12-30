// Generated macro for impl_1059 (impl)
macro_rules! Depcrate_collections_vec_dequeimpl_1059 {
() => {
// Module: crate::collections::vec_deque
// Provides: {"impl_1059"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T , A : Allocator > Index < usize > for VecDeque < T , A > { type Output = T ; # [inline] fn index (& self , index : usize) -> & T { self . get (index) . expect ("Out of bounds access") } }
};
}
