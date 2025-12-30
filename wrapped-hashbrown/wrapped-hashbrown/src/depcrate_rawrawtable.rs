// Generated macro for RawTable (struct)
macro_rules! Depcrate_rawRawTable {
() => {
// Module: crate::raw
// Provides: {"RawTable"}
// Dependencies: {}
# [doc = " A raw hash table with an unsafe API."] pub struct RawTable < T , A : Allocator = Global > { table : RawTableInner , alloc : A , marker : PhantomData < T > , }
};
}
