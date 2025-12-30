// Generated macro for RawIntoIter (struct)
macro_rules! Depcrate_rawRawIntoIter {
() => {
// Module: crate::raw
// Provides: {"RawIntoIter"}
// Dependencies: {}
# [doc = " Iterator which consumes a table and returns elements."] pub struct RawIntoIter < T , A : Allocator = Global > { iter : RawIter < T > , allocation : Option < (NonNull < u8 > , Layout , A) > , marker : PhantomData < T > , }
};
}
