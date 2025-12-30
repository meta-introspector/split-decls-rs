// Generated macro for Encoding (trait)
macro_rules! Depcrate_traitsEncoding {
() => {
// Module: crate::traits
// Provides: {"Encoding"}
// Dependencies: {}
# [doc = " Encoding support."] pub trait Encoding : Sized { # [doc = " Byte array representation."] type Repr : AsRef < [u8] > + AsMut < [u8] > + Copy + Clone + Sized + for < 'a > TryFrom < & 'a [u8] , Error = core :: array :: TryFromSliceError > ; # [doc = " Decode from big endian bytes."] fn from_be_bytes (bytes : Self :: Repr) -> Self ; # [doc = " Decode from little endian bytes."] fn from_le_bytes (bytes : Self :: Repr) -> Self ; # [doc = " Encode to big endian bytes."] fn to_be_bytes (& self) -> Self :: Repr ; # [doc = " Encode to little endian bytes."] fn to_le_bytes (& self) -> Self :: Repr ; }
};
}
