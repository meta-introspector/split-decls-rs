macro_rules! ChecksumCalculator {
    () => {
        # [doc = " Handles checksum calculation for different XZ check types."] enum ChecksumCalculator { None , Crc32 (crc :: Digest < 'static , u32 >) , Crc64 (crc :: Digest < 'static , u64 >) , Sha256 (sha2 :: Sha256) , }
    };
}

ChecksumCalculator!()