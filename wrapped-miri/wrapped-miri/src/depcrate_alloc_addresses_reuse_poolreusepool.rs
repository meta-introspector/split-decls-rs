// Generated macro for ReusePool (struct)
macro_rules! Depcrate_alloc_addresses_reuse_poolReusePool {
() => {
// Module: crate::alloc_addresses::reuse_pool
// Provides: {"ReusePool"}
// Dependencies: {}
# [doc = " The pool strikes a balance between exploring more possible executions and making it more likely"] # [doc = " to find bugs. The hypothesis is that bugs are more likely to occur when reuse happens for"] # [doc = " allocations with the same layout, since that can trigger e.g. ABA issues in a concurrent data"] # [doc = " structure. Therefore we only reuse allocations when size and alignment match exactly."] # [derive (Debug)] pub struct ReusePool { address_reuse_rate : f64 , address_reuse_cross_thread_rate : f64 , # [doc = " The i-th element in `pool` stores allocations of alignment `2^i`. We store these reusable"] # [doc = " allocations as address-size pairs, the list must be sorted by the size and then the thread ID."] # [doc = ""] # [doc = " Each of these maps has at most MAX_POOL_SIZE elements, and since alignment is limited to"] # [doc = " less than 64 different possible values, that bounds the overall size of the pool."] # [doc = ""] # [doc = " We also store the ID and the data-race clock of the thread that donated this pool element,"] # [doc = " to ensure synchronization with the thread that picks up this address."] pool : Vec < Vec < (u64 , Size , ThreadId , VClock) > > , }
};
}
