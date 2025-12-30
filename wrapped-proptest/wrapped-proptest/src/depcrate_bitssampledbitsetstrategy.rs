// Generated macro for SampledBitSetStrategy (struct)
macro_rules! Depcrate_bitsSampledBitSetStrategy {
() => {
// Module: crate::bits
// Provides: {"SampledBitSetStrategy"}
// Dependencies: {}
# [doc = " Generates bit sets with a particular number of bits set."] # [doc = ""] # [doc = " Specifically, this strategy is given both a size range and a bit range. To"] # [doc = " produce a new value, it selects a size, then uniformly selects that many"] # [doc = " bits from within the bit range."] # [doc = ""] # [doc = " Shrinking happens as with [`BitSetStrategy`](struct.BitSetStrategy.html)."] # [derive (Clone , Debug)] # [must_use = "strategies do nothing unless used"] pub struct SampledBitSetStrategy < T : BitSetLike > { size : SizeRange , bits : SizeRange , _marker : PhantomData < T > , }
};
}
