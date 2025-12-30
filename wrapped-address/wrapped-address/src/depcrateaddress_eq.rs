// Generated macro for address_eq (function)
macro_rules! Depcrateaddress_eq {
() => {
// Module: crate
// Provides: {"address_eq"}
// Dependencies: {}
# [doc = " Custom implementation of equality for `Address`."] # [doc = ""] # [doc = " The implementation compares the address in 4 chunks of 8 bytes (`u64` values),"] # [doc = " which is currently more efficient (CU-wise) than the default implementation."] # [doc = ""] # [doc = " This isn't the implementation for the `PartialEq` trait because we can't do"] # [doc = " structural equality with a trait implementation."] # [doc = ""] # [doc = " [Issue #345](https://github.com/anza-xyz/solana-sdk/issues/345) contains"] # [doc = " more information about the problem."] # [inline (always)] pub fn address_eq (a1 : & Address , a2 : & Address) -> bool { let p1_ptr = a1 . 0 . as_ptr () . cast :: < u64 > () ; let p2_ptr = a2 . 0 . as_ptr () . cast :: < u64 > () ; unsafe { read_unaligned (p1_ptr) == read_unaligned (p2_ptr) && read_unaligned (p1_ptr . add (1)) == read_unaligned (p2_ptr . add (1)) && read_unaligned (p1_ptr . add (2)) == read_unaligned (p2_ptr . add (2)) && read_unaligned (p1_ptr . add (3)) == read_unaligned (p2_ptr . add (3)) } }
};
}
