macro_rules! MH_ROOT_SAFE {
    () => {
        # [doc = " When this bit is set, the binary declares it is safe for use in processes with uid zero"] pub const MH_ROOT_SAFE : u32 = 0x40000 ;
    };
}

MH_ROOT_SAFE!()