macro_rules! update_slow {
    () => {
        pub (crate) fn update_slow (prev : u32 , buf : & [u8]) -> u32 { let mut crc = ! prev ; for & byte in buf . iter () { crc = CRC32_TABLE [0] [((crc as u8) ^ byte) as usize] ^ (crc >> 8) ; } ! crc }
    };
}

update_slow!()