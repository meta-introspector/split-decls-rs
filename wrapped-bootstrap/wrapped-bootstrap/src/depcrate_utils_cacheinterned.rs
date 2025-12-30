// Generated macro for Interned (struct)
macro_rules! Depcrate_utils_cacheInterned {
() => {
// Module: crate::utils::cache
// Provides: {"Interned"}
// Dependencies: {}
# [doc = " Represents an interned value of type `T`, allowing for efficient comparisons and retrieval."] # [doc = ""] # [doc = " This struct stores a unique index referencing the interned value within an internal cache."] pub struct Interned < T > (usize , PhantomData < * const T >) ;
};
}
