// Generated macro for impl_12 (impl)
macro_rules! Depcrate_ascii_setimpl_12 {
() => {
// Module: crate::ascii_set
// Provides: {"impl_12"}
// Dependencies: {}
impl AsciiSet { # [doc = " An empty set."] pub const EMPTY : Self = Self { mask : [0 ; ASCII_RANGE_LEN / BITS_PER_CHUNK] , } ; # [doc = " Called with UTF-8 bytes rather than code points."] # [doc = " Not used for non-ASCII bytes."] pub (crate) const fn contains (& self , byte : u8) -> bool { let chunk = self . mask [byte as usize / BITS_PER_CHUNK] ; let mask = 1 << (byte as usize % BITS_PER_CHUNK) ; (chunk & mask) != 0 } pub (crate) fn should_percent_encode (& self , byte : u8) -> bool { ! byte . is_ascii () || self . contains (byte) } pub const fn add (& self , byte : u8) -> Self { let mut mask = self . mask ; mask [byte as usize / BITS_PER_CHUNK] |= 1 << (byte as usize % BITS_PER_CHUNK) ; Self { mask } } pub const fn remove (& self , byte : u8) -> Self { let mut mask = self . mask ; mask [byte as usize / BITS_PER_CHUNK] &= ! (1 << (byte as usize % BITS_PER_CHUNK)) ; Self { mask } } # [doc = " Return the union of two sets."] pub const fn union (& self , other : Self) -> Self { let mask = [self . mask [0] | other . mask [0] , self . mask [1] | other . mask [1] , self . mask [2] | other . mask [2] , self . mask [3] | other . mask [3] ,] ; Self { mask } } # [doc = " Return the negation of the set."] pub const fn complement (& self) -> Self { let mask = [! self . mask [0] , ! self . mask [1] , ! self . mask [2] , ! self . mask [3]] ; Self { mask } } }
};
}
