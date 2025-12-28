macro_rules! NT_PLATFORM {
    () => {
        # [doc = " String from sysinfo(SI_PLATFORM)."] pub const NT_PLATFORM : u32 = 5 ;
    };
}

NT_PLATFORM!();