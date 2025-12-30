// Generated macro for RawDrain (struct)
macro_rules! Depcrate_rawRawDrain {
() => {
// Module: crate::raw
// Provides: {"RawDrain"}
// Dependencies: {}
# [doc = " Iterator which consumes elements without freeing the table storage."] pub struct RawDrain < 'a , T , A : Allocator = Global > { iter : RawIter < T > , table : RawTableInner , orig_table : NonNull < RawTableInner > , marker : PhantomData < & 'a RawTable < T , A > > , }
};
}
