macro_rules! deps {
    () => {
        LzEncoder!();
        Hash234!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl Hash234 { fn get_hash4_size (dict_size : u32) -> u32 { let mut h = dict_size - 1 ; h |= h >> 1 ; h |= h >> 2 ; h |= h >> 4 ; h |= h >> 8 ; h >>= 1 ; h |= 0xFFFF ; if h > (1 << 24) { h >>= 1 ; } h + 1 } pub (crate) fn get_mem_usage (dict_size : u32) -> u32 { (HASH2_MASK + HASH2_SIZE + Self :: get_hash4_size (dict_size)) / (1024 / 4) + 4 } pub (crate) fn new (dict_size : u32) -> Self { let hash4_size = Self :: get_hash4_size (dict_size) ; let hash4_mask = hash4_size - 1 ; let hash2_table = vec ! [0 ; HASH2_SIZE as usize] ; let hash3_table = vec ! [0 ; HASH3_SIZE as usize] ; let hash4_table = vec ! [0 ; hash4_size as usize] ; Self { hash4_mask , hash2_table , hash3_table , hash4_table , hash4_size , hash2_value : 0 , hash3_value : 0 , hash4_value : 0 , } } # [inline (always)] fn hash_byte (byte : u8) -> u32 { (byte as u32) . wrapping_mul (0x9E3779B9) } # [inline (always)] pub (crate) fn calc_hashes (& mut self , buf : & [u8]) { let tmp = CRC_TABLE [buf [0] as usize] ^ (buf [1] as u32) ; self . hash2_value = (tmp & HASH2_MASK) as i32 ; let tmp = tmp ^ ((buf [2] as u32) << 8) ; self . hash3_value = (tmp & HASH3_MASK) as i32 ; let tmp = tmp ^ (CRC_TABLE [buf [3] as usize] << 5) ; self . hash4_value = (tmp & self . hash4_mask) as i32 ; } pub (crate) fn get_hash2_pos (& self) -> i32 { self . hash2_table [self . hash2_value as usize] } pub (crate) fn get_hash3_pos (& self) -> i32 { self . hash3_table [self . hash3_value as usize] } pub (crate) fn get_hash4_pos (& self) -> i32 { self . hash4_table [self . hash4_value as usize] } pub (crate) fn update_tables (& mut self , pos : i32) { self . hash2_table [self . hash2_value as usize] = pos ; self . hash3_table [self . hash3_value as usize] = pos ; self . hash4_table [self . hash4_value as usize] = pos ; } pub (crate) fn normalize (& mut self , offset : i32) { LzEncoder :: normalize (& mut self . hash2_table , offset) ; LzEncoder :: normalize (& mut self . hash3_table , offset) ; LzEncoder :: normalize (& mut self . hash4_table , offset) ; } }
    };
}

impl_20!();