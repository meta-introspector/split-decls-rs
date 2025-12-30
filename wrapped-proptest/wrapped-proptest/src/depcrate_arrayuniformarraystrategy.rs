// Generated macro for UniformArrayStrategy (struct)
macro_rules! Depcrate_arrayUniformArrayStrategy {
() => {
// Module: crate::array
// Provides: {"UniformArrayStrategy"}
// Dependencies: {}
# [doc = " A `Strategy` which generates fixed-size arrays containing values drawn from"] # [doc = " an inner strategy."] # [doc = ""] # [doc = " `T` must be an array type of length 1 to 32 whose values are produced by"] # [doc = " strategy `S`. Instances of this type are normally created by the various"] # [doc = " `uniformXX` functions in this module."] # [doc = ""] # [doc = " This is mainly useful when the inner strategy is not `Copy`, precluding"] # [doc = " expressing the strategy as `[myStrategy; 32]`, for example."] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [doc = " ```"] # [doc = " use proptest::prelude::*;"] # [doc = ""] # [doc = " proptest! {"] # [doc = "   #[test]"] # [doc = "   fn test_something(a in prop::array::uniform32(1u32..)) {"] # [doc = "     let unexpected = [0u32;32];"] # [doc = "     // `a` is also a [u32;32], so we can compare them directly"] # [doc = "     assert_ne!(unexpected, a);"] # [doc = "   }"] # [doc = " }"] # [doc = " # fn main() { }"] # [doc = " ```"] # [must_use = "strategies do nothing unless used"] # [derive (Clone , Copy , Debug)] pub struct UniformArrayStrategy < S , T > { strategy : S , _marker : PhantomData < T > , }
};
}
