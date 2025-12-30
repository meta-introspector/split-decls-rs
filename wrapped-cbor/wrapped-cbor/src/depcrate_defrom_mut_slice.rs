// Generated macro for from_mut_slice (function)
macro_rules! Depcrate_defrom_mut_slice {
() => {
// Module: crate::de
// Provides: {"from_mut_slice"}
// Dependencies: {}
# [doc = " Decode a value from CBOR data in a mutable slice."] # [doc = ""] # [doc = " This can be used in analogy to `from_slice`. Unlike `from_slice`, this will use the slice's"] # [doc = " mutability to rearrange data in it in order to resolve indefinite byte or text strings without"] # [doc = " resorting to allocations."] pub fn from_mut_slice < 'a , T > (slice : & 'a mut [u8]) -> Result < T > where T : de :: Deserialize < 'a > , { let mut deserializer = Deserializer :: from_mut_slice (slice) ; let value = de :: Deserialize :: deserialize (& mut deserializer) ? ; deserializer . end () ? ; Ok (value) }
};
}
