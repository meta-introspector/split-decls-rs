// Generated macro for ChecksumCalculator (enum)
macro_rules! Depcrate_xzChecksumCalculator {
() => {
// Module: crate::xz
// Provides: {"ChecksumCalculator"}
// Dependencies: {}
# [doc = " Handles checksum calculation for different XZ check types."] enum ChecksumCalculator { None , Crc32 (crc :: Digest < 'static , u32 >) , Crc64 (crc :: Digest < 'static , u64 >) , Sha256 (sha2 :: Sha256) , }
};
}
