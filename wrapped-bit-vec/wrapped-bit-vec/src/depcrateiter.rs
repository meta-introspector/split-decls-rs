// Generated macro for Iter (struct)
macro_rules! DepcrateIter {
() => {
// Module: crate
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " An iterator for `BitVec`."] # [derive (Clone)] pub struct Iter < 'a , B : 'a = u32 > { bit_vec : & 'a BitVec < B > , range : Range < usize > , }
};
}
