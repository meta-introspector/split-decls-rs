// Generated macro for impl_627 (impl)
macro_rules! Depcrate_bitsimpl_627 {
() => {
// Module: crate::bits
// Provides: {"impl_627"}
// Dependencies: {}
impl < T : BitSetLike > BitSetStrategy < T > { # [doc = " Create a strategy which generates values where bits between `min`"] # [doc = " (inclusive) and `max` (exclusive) may be set."] # [doc = ""] # [doc = " Due to the generics, the functions in the typed submodules are usually"] # [doc = " preferable to calling this directly."] pub fn new (min : usize , max : usize) -> Self { BitSetStrategy { min , max , mask : None , } } # [doc = " Create a strategy which generates values where any bits set (and only"] # [doc = " those bits) in `mask` may be set."] pub fn masked (mask : T) -> Self { BitSetStrategy { min : 0 , max : mask . len () , mask : Some (mask) , } } }
};
}
