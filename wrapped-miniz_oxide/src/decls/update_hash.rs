macro_rules! update_hash {
    () => {
        # [inline] pub const fn update_hash (current_hash : u16 , byte : u8) -> u16 { ((current_hash << LZ_HASH_SHIFT) ^ byte as u16) & (LZ_HASH_SIZE as u16 - 1) }
    };
}

update_hash!()