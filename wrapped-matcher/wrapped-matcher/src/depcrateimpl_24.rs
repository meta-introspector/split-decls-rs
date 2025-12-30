// Generated macro for impl_24 (impl)
macro_rules! Depcrateimpl_24 {
() => {
// Module: crate
// Provides: {"impl_24"}
// Dependencies: {}
impl ByteSet { # [doc = " Create an empty set of bytes."] # [inline] pub fn empty () -> ByteSet { ByteSet (BitSet ([0 ; 4])) } # [doc = " Create a full set of bytes such that every possible byte is in the set"] # [doc = " returned."] # [inline] pub fn full () -> ByteSet { ByteSet (BitSet ([u64 :: MAX ; 4])) } # [doc = " Add a byte to this set."] # [doc = ""] # [doc = " If the given byte already belongs to this set, then this is a no-op."] # [inline] pub fn add (& mut self , byte : u8) { let bucket = byte / 64 ; let bit = byte % 64 ; (self . 0) . 0 [usize :: from (bucket)] |= 1 << bit ; } # [doc = " Add an inclusive range of bytes."] # [inline] pub fn add_all (& mut self , start : u8 , end : u8) { for b in start ..= end { self . add (b) ; } } # [doc = " Remove a byte from this set."] # [doc = ""] # [doc = " If the given byte is not in this set, then this is a no-op."] # [inline] pub fn remove (& mut self , byte : u8) { let bucket = byte / 64 ; let bit = byte % 64 ; (self . 0) . 0 [usize :: from (bucket)] &= ! (1 << bit) ; } # [doc = " Remove an inclusive range of bytes."] # [inline] pub fn remove_all (& mut self , start : u8 , end : u8) { for b in start ..= end { self . remove (b) ; } } # [doc = " Return true if and only if the given byte is in this set."] # [inline] pub fn contains (& self , byte : u8) -> bool { let bucket = byte / 64 ; let bit = byte % 64 ; (self . 0) . 0 [usize :: from (bucket)] & (1 << bit) > 0 } }
};
}
