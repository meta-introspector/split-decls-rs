// Generated macro for impl_76 (impl)
macro_rules! Depcrate_rawimpl_76 {
() => {
// Module: crate::raw
// Provides: {"impl_76"}
// Dependencies: {}
impl < T > RawTable < T , Global > { # [doc = " Creates a new empty hash table without allocating any memory."] # [doc = ""] # [doc = " In effect this returns a table with exactly 1 bucket. However we can"] # [doc = " leave the data pointer dangling since that bucket is never written to"] # [doc = " due to our load factor forcing us to always have at least 1 free bucket."] # [inline] # [cfg_attr (feature = "rustc-dep-of-std" , rustc_const_stable_indirect)] pub const fn new () -> Self { Self { table : RawTableInner :: NEW , alloc : Global , marker : PhantomData , } } # [doc = " Allocates a new hash table with at least enough capacity for inserting"] # [doc = " the given number of elements without reallocating."] pub fn with_capacity (capacity : usize) -> Self { Self :: with_capacity_in (capacity , Global) } }
};
}
