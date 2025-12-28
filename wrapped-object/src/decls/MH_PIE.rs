macro_rules! MH_PIE {
    () => {
        # [doc = " When this bit is set, the OS will load the main executable at a random address.  Only used in MH_EXECUTE filetypes."] pub const MH_PIE : u32 = 0x20_0000 ;
    };
}

MH_PIE!()