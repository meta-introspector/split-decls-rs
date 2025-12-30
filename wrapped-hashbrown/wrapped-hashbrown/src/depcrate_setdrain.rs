// Generated macro for Drain (struct)
macro_rules! Depcrate_setDrain {
() => {
// Module: crate::set
// Provides: {"Drain"}
// Dependencies: {}
# [doc = " A draining iterator over the items of a `HashSet`."] # [doc = ""] # [doc = " This `struct` is created by the [`drain`] method on [`HashSet`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`HashSet`]: struct.HashSet.html"] # [doc = " [`drain`]: struct.HashSet.html#method.drain"] pub struct Drain < 'a , K , A : Allocator = Global > { iter : map :: Drain < 'a , K , () , A > , }
};
}
