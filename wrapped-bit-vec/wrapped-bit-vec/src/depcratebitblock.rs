// Generated macro for BitBlock (trait)
macro_rules! DepcrateBitBlock {
() => {
// Module: crate
// Provides: {"BitBlock"}
// Dependencies: {}
# [doc = " Abstracts over a pile of bits (basically unsigned primitives)"] pub trait BitBlock : Copy + Add < Self , Output = Self > + Sub < Self , Output = Self > + Shl < usize , Output = Self > + Shr < usize , Output = Self > + Not < Output = Self > + BitAnd < Self , Output = Self > + BitOr < Self , Output = Self > + BitXor < Self , Output = Self > + Rem < Self , Output = Self > + Eq + Ord + hash :: Hash { # [doc = " How many bits it has"] fn bits () -> usize ; # [doc = " How many bytes it has"] # [inline] fn bytes () -> usize { Self :: bits () / 8 } # [doc = " Convert a byte into this type (lowest-order bits set)"] fn from_byte (byte : u8) -> Self ; # [doc = " Count the number of 1's in the bitwise repr"] fn count_ones (self) -> usize ; # [doc = " Count the number of 0's in the bitwise repr"] fn count_zeros (self) -> usize { Self :: bits () - self . count_ones () } # [doc = " Get `0`"] fn zero () -> Self ; # [doc = " Get `1`"] fn one () -> Self ; }
};
}
