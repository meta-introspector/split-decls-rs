// Generated macro for impl_23 (impl)
macro_rules! Depcrateimpl_23 {
() => {
// Module: crate
// Provides: {"impl_23"}
// Dependencies: {}
impl BitSet < u32 > { # [doc = " Creates a new empty `BitSet`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use bit_set::BitSet;"] # [doc = ""] # [doc = " let mut s = BitSet::new();"] # [doc = " ```"] # [inline] pub fn new () -> Self { Self :: default () } # [doc = " Creates a new `BitSet` with initially no contents, able to"] # [doc = " hold `nbits` elements without resizing."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use bit_set::BitSet;"] # [doc = ""] # [doc = " let mut s = BitSet::with_capacity(100);"] # [doc = " assert!(s.capacity() >= 100);"] # [doc = " ```"] # [inline] pub fn with_capacity (nbits : usize) -> Self { let bit_vec = BitVec :: from_elem (nbits , false) ; Self :: from_bit_vec (bit_vec) } # [doc = " Creates a new `BitSet` from the given bit vector."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " fn main() {"] # [doc = "     use bit_vec::BitVec;"] # [doc = "     use bit_set::BitSet;"] # [doc = ""] # [doc = "     let bv = BitVec::from_bytes(&[0b01100000]);"] # [doc = "     let s = BitSet::from_bit_vec(bv);"] # [doc = ""] # [doc = "     // Print 1, 2 in arbitrary order"] # [doc = "     for x in s.iter() {"] # [doc = "         println!(\"{}\", x);"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [inline] pub fn from_bit_vec (bit_vec : BitVec) -> Self { BitSet { bit_vec } } pub fn from_bytes (bytes : & [u8]) -> Self { BitSet { bit_vec : BitVec :: from_bytes (bytes) , } } }
};
}
