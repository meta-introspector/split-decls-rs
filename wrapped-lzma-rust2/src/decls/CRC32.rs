macro_rules! CRC32 {
    () => {
        const CRC32 : crc :: Crc < u32 > = crc :: Crc :: < u32 > :: new (& crc :: CRC_32_ISO_HDLC) ;
    };
}

CRC32!()