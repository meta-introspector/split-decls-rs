// Generated macro for test_alloc (module)
macro_rules! Depcrate_detest_alloc {
() => {
// Module: crate::de
// Provides: {"test_alloc"}
// Dependencies: {}
# [cfg (any (feature = "alloc" , feature = "use-std"))] # [cfg (test)] mod test_alloc { extern crate alloc ; use super :: * ; use alloc :: vec ; use serde :: Deserialize ; # [derive (Debug , Deserialize , PartialEq)] struct ZSTStruct ; # [test] fn zst_vec () { assert_eq ! (from_bytes (& [3]) , Ok (vec ! [ZSTStruct , ZSTStruct , ZSTStruct])) ; assert_eq ! (from_bytes (& [4]) , Ok (vec ! [ZSTStruct , ZSTStruct , ZSTStruct , ZSTStruct])) ; } # [test] fn vec () { assert_eq ! (from_bytes ::< Vec < u8 >> (& [8 , 255 , 255 , 255 , 0 , 0 , 0 , 0 , 0]) , Ok (vec ! [255 , 255 , 255 , 0 , 0 , 0 , 0 , 0])) ; assert_eq ! (from_bytes ::< Vec < u8 >> (& [(1 << 7) | 8 , 255 , 255 , 255 , 0 , 0 , 0 , 0 , 0]) , Err (Error :: DeserializeUnexpectedEnd)) ; } }
};
}
