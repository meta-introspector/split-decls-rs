// Generated macro for from_slice_with_scratch (function)
macro_rules! Depcrate_defrom_slice_with_scratch {
() => {
// Module: crate::de
// Provides: {"from_slice_with_scratch"}
// Dependencies: {}
# [doc = " Decode a value from CBOR data using a scratch buffer."] # [doc = ""] # [doc = " Users should generally prefer to use `from_slice` or `from_mut_slice` over this function,"] # [doc = " as decoding may fail when the scratch buffer turns out to be too small."] # [doc = ""] # [doc = " A realistic use case for this method would be decoding in a `no_std` environment from an"] # [doc = " immutable slice that is too large to copy."] pub fn from_slice_with_scratch < 'a , 'b , T > (slice : & 'a [u8] , scratch : & 'b mut [u8]) -> Result < T > where T : de :: Deserialize < 'a > , { let mut deserializer = Deserializer :: from_slice_with_scratch (slice , scratch) ; let value = de :: Deserialize :: deserialize (& mut deserializer) ? ; deserializer . end () ? ; Ok (value) }
};
}
