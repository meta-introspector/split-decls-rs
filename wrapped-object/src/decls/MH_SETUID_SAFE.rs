macro_rules! MH_SETUID_SAFE {
    () => {
        # [doc = " When this bit is set, the binary declares it is safe for use in processes when issetugid() is true"] pub const MH_SETUID_SAFE : u32 = 0x80000 ;
    };
}

MH_SETUID_SAFE!();