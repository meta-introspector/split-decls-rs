macro_rules! Nlist {
    () => {
        struct Nlist { n_strx : u32 , n_type : u8 , n_sect : u8 , n_desc : u16 , n_value : u64 , }
    };
}

Nlist!();