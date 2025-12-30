// Generated macro for impl_41 (impl)
macro_rules! Depcrate_digestimpl_41 {
() => {
// Module: crate::digest
// Provides: {"impl_41"}
// Dependencies: {}
impl PartialEq for Digest16 { fn eq (& self , other : & Self) -> bool { let a = & self . 0 ; let b = & other . 0 ; (u8 :: from (a [0] != b [0]) + u8 :: from (a [1] != b [1]) + u8 :: from (a [2] != b [2]) + u8 :: from (a [3] != b [3]) + u8 :: from (a [4] != b [4]) + u8 :: from (a [5] != b [5]) + u8 :: from (a [6] != b [6]) + u8 :: from (a [7] != b [7]) + u8 :: from (a [8] != b [8]) + u8 :: from (a [9] != b [9]) + u8 :: from (a [10] != b [10]) + u8 :: from (a [11] != b [11]) + u8 :: from (a [12] != b [12]) + u8 :: from (a [13] != b [13]) + u8 :: from (a [14] != b [14]) + u8 :: from (a [15] != b [15])) == 0u8 } }
};
}
