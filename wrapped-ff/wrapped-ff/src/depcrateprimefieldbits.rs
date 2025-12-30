// Generated macro for PrimeFieldBits (trait)
macro_rules! DepcratePrimeFieldBits {
() => {
// Module: crate
// Provides: {"PrimeFieldBits"}
// Dependencies: {}
# [doc = " This represents the bits of an element of a prime field."] # [cfg (feature = "bits")] # [cfg_attr (docsrs , doc (cfg (feature = "bits")))] pub trait PrimeFieldBits : PrimeField { # [doc = " The backing store for a bit representation of a prime field element."] type ReprBits : BitViewSized + Send + Sync ; # [doc = " Converts an element of the prime field into a little-endian sequence of bits."] fn to_le_bits (& self) -> FieldBits < Self :: ReprBits > ; # [doc = " Returns the bits of the field characteristic (the modulus) in little-endian order."] fn char_le_bits () -> FieldBits < Self :: ReprBits > ; }
};
}
