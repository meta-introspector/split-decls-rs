// Generated macro for test (module)
macro_rules! Depcratetest {
() => {
// Module: crate
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: { Crc , CRC_32_ISCSI } ; # [test] fn test_clone () { const CRC : Crc < u32 > = Crc :: < u32 > :: new (& CRC_32_ISCSI) ; let crc = CRC . clone () ; let digest = crc . digest () ; let _digest = digest . clone () ; } }
};
}
