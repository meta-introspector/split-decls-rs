macro_rules! ChecksumType {
    () => {
        # [doc = " Used by BlockBasedOptions::set_checksum_type."] pub enum ChecksumType { NoChecksum = 0 , CRC32c = 1 , XXHash = 2 , XXHash64 = 3 , XXH3 = 4 , }
    };
}

ChecksumType!()