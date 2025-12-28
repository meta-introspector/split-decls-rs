macro_rules! CRC64 {
    () => {
        const CRC64 : crc :: Crc < u64 > = crc :: Crc :: < u64 > :: new (& crc :: CRC_64_XZ) ;
    };
}

CRC64!();