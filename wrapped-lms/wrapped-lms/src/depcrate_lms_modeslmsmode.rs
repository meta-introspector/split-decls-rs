// Generated macro for LmsMode (trait)
macro_rules! Depcrate_lms_modesLmsMode {
() => {
// Module: crate::lms::modes
// Provides: {"LmsMode"}
// Dependencies: {}
# [doc = " The basic trait that must be implemented for any valid LMS mode"] pub trait LmsMode : Typecode + Clone { # [doc = " The underlying hash function"] type Hasher : Digest ; # [doc = " The underlying LM-OTS mode"] type OtsMode : LmsOtsMode ; # [doc = " Length of the internal Merkle tree, computed as `2^(h+1)-1`"] type TreeLen : ArraySize ; # [doc = " `h` as a type"] type HLen : ArraySize ; # [doc = " The length of the hash function output as a type"] const M : usize ; # [doc = " `h` as a [usize]"] const H : usize ; # [doc = " The number of leaves as a [u32], computed as `2^h`"] const LEAVES : u32 ; # [doc = " `TreeLen` as a [u32], `2^(h+1)-1`"] const TREE_NODES : u32 ; }
};
}
