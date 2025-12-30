// Generated macro for tests (module)
macro_rules! Depcrate_endiantests {
() => {
// Module: crate::endian
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_big_endian () { let x = BigEndian :: from (1u32) ; let x2 = x ; assert_eq ! (u32 :: from (x) , 1) ; assert_eq ! (u32 :: from (x2) , 1) ; } # [test] fn test_endian_from_array () { let be : [BigEndian < u32 > ; 2] = BigEndian :: < u32 > :: from_array (& [0x_AABB_CCDD_u32 , 0x_2233_4455_u32]) ; let le : [LittleEndian < u32 > ; 2] = LittleEndian :: < u32 > :: from_array (& [0x_DDCC_BBAA_u32 , 0x_5544_3322_u32]) ; assert_eq ! (be . as_byte_array () , le . as_byte_array ()) ; let be : [BigEndian < u64 > ; 2] = BigEndian :: < u64 > :: from_array (& [0x_AABB_CCDD_EEFF_0011_u64 , 0x_2233_4455_6677_8899_u64]) ; let le : [LittleEndian < u64 > ; 2] = LittleEndian :: < u64 > :: from_array (& [0x_1100_FFEE_DDCC_BBAA_u64 , 0x_9988_7766_5544_3322_u64 ,]) ; assert_eq ! (be . as_byte_array () , le . as_byte_array ()) ; } }
};
}
