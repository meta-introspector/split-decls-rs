// Generated macro for impl_488 (impl)
macro_rules! Depcrate_vector_poolimpl_488 {
() => {
// Module: crate::vector::pool
// Provides: {"impl_488"}
// Dependencies: {}
impl < A > RRBPool < A > { # [doc = " Create a new memory pool with the given size."] pub fn new (size : usize) -> Self { Self :: with_sizes (size , size , size) } # [doc = " Create a new memory pool with the given sizes for each subpool."] pub fn with_sizes (node_pool_size : usize , leaf_pool_size : usize , size_table_pool_size : usize ,) -> Self { Self { node_pool : Pool :: new (node_pool_size) , value_pool : Pool :: new (leaf_pool_size) , size_pool : Pool :: new (size_table_pool_size) , } } # [doc = " Fill the memory pool with preallocated chunks."] pub fn fill (& self) { self . node_pool . fill () ; self . value_pool . fill () ; self . size_pool . fill () ; } # [doc = " Get the size of the node subpool."] pub fn node_pool_size (& self) -> usize { self . node_pool . get_pool_size () } # [doc = " Get the size of the leaf node subpool."] pub fn leaf_pool_size (& self) -> usize { self . value_pool . get_pool_size () } # [doc = " Get the size of the size table subpool."] pub fn size_table_pool_size (& self) -> usize { self . size_pool . get_pool_size () } }
};
}
