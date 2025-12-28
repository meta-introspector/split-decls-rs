macro_rules! SG_FVMLIB {
    () => {
        # [doc = " this segment is the VM that is allocated by a fixed VM library, for overlap checking in the link editor"] pub const SG_FVMLIB : u32 = 0x2 ;
    };
}

SG_FVMLIB!();