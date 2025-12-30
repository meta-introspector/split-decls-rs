// Generated macro for impl_630 (impl)
macro_rules! Depcrate_bitsimpl_630 {
() => {
// Module: crate::bits
// Provides: {"impl_630"}
// Dependencies: {}
impl < T : BitSetLike > SampledBitSetStrategy < T > { # [doc = " Create a strategy which generates values where bits within the bounds"] # [doc = " given by `bits` may be set. The number of bits that are set is chosen"] # [doc = " to be in the range given by `size`."] # [doc = ""] # [doc = " Due to the generics, the functions in the typed submodules are usually"] # [doc = " preferable to calling this directly."] # [doc = ""] # [doc = " ## Panics"] # [doc = ""] # [doc = " Panics if `size` includes a value that is greater than the number of"] # [doc = " bits in `bits`."] pub fn new (size : impl Into < SizeRange > , bits : impl Into < SizeRange >) -> Self { let size = size . into () ; let bits = bits . into () ; size . assert_nonempty () ; let available_bits = bits . end_excl () - bits . start () ; assert ! (size . end_excl () <= available_bits + 1 , "Illegal SampledBitSetStrategy: have {} bits available, \
             but requested size is {}..{}" , available_bits , size . start () , size . end_excl ()) ; SampledBitSetStrategy { size , bits , _marker : PhantomData , } } }
};
}
