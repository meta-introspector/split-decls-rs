macro_rules! C_MAX {
    () => {
        # [doc = " The maximum size of the ciphertext."] pub const C_MAX : u64 = P_MAX + (TAG_SIZE as u64) ;
    };
}

C_MAX!();