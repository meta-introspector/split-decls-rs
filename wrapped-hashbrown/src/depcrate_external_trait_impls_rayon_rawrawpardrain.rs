// Generated macro for RawParDrain (struct)
macro_rules! Depcrate_external_trait_impls_rayon_rawRawParDrain {
() => {
// Module: crate::external_trait_impls::rayon::raw
// Provides: {"RawParDrain"}
// Dependencies: {}
# [doc = " Parallel iterator which consumes elements without freeing the table storage."] pub struct RawParDrain < 'a , T , A : Allocator = Global > { table : NonNull < RawTable < T , A > > , marker : PhantomData < & 'a RawTable < T , A > > , }
};
}
