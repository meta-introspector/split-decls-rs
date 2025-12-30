// Generated macro for int_api (macro)
macro_rules! Depcrate_bitsint_api {
() => {
// Module: crate::bits
// Provides: {"int_api"}
// Dependencies: {}
macro_rules ! int_api { ($ typ : ident , $ max : expr) => { # [allow (missing_docs)] pub mod $ typ { use super ::*; # [doc = " Generates integers where all bits may be set."] pub const ANY : BitSetStrategy <$ typ > = BitSetStrategy { min : 0 , max : $ max , mask : None , } ; # [doc = " Generates values where bits between the given bounds may be"] # [doc = " set."] pub fn between (min : usize , max : usize) -> BitSetStrategy <$ typ > { BitSetStrategy :: new (min , max) } # [doc = " Generates values where any bits set in `mask` (and no others)"] # [doc = " may be set."] pub fn masked (mask : $ typ) -> BitSetStrategy <$ typ > { BitSetStrategy :: masked (mask) } # [doc = " Create a strategy which generates values where bits within the"] # [doc = " bounds given by `bits` may be set. The number of bits that are"] # [doc = " set is chosen to be in the range given by `size`."] # [doc = ""] # [doc = " ## Panics"] # [doc = ""] # [doc = " Panics if `size` includes a value that is greater than the"] # [doc = " number of bits in `bits`."] pub fn sampled (size : impl Into < SizeRange >, bits : impl Into < SizeRange >,) -> SampledBitSetStrategy <$ typ > { SampledBitSetStrategy :: new (size , bits) } } } ; }
};
}
