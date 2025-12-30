// Generated macro for BitVec (struct)
macro_rules! DepcrateBitVec {
() => {
// Module: crate
// Provides: {"BitVec"}
// Dependencies: {}
# [doc = " The bitvector type."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use bit_vec::BitVec;"] # [doc = ""] # [doc = " let mut bv = BitVec::from_elem(10, false);"] # [doc = ""] # [doc = " // insert all primes less than 10"] # [doc = " bv.set(2, true);"] # [doc = " bv.set(3, true);"] # [doc = " bv.set(5, true);"] # [doc = " bv.set(7, true);"] # [doc = " println!(\"{:?}\", bv);"] # [doc = " println!(\"total bits set to true: {}\", bv.iter().filter(|x| *x).count());"] # [doc = ""] # [doc = " // flip all values in bitvector, producing non-primes less than 10"] # [doc = " bv.negate();"] # [doc = " println!(\"{:?}\", bv);"] # [doc = " println!(\"total bits set to true: {}\", bv.iter().filter(|x| *x).count());"] # [doc = ""] # [doc = " // reset bitvector to empty"] # [doc = " bv.clear();"] # [doc = " println!(\"{:?}\", bv);"] # [doc = " println!(\"total bits set to true: {}\", bv.iter().filter(|x| *x).count());"] # [doc = " ```"] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] # [cfg_attr (feature = "borsh" , derive (borsh :: BorshDeserialize , borsh :: BorshSerialize))] # [cfg_attr (feature = "miniserde" , derive (miniserde :: Deserialize , miniserde :: Serialize))] # [cfg_attr (feature = "nanoserde" , derive (DeBin , DeJson , DeRon , SerBin , SerJson , SerRon))] pub struct BitVec < B = u32 > { # [doc = " Internal representation of the bit vector"] storage : Vec < B > , # [doc = " The number of valid bits in the internal representation"] nbits : usize , }
};
}
