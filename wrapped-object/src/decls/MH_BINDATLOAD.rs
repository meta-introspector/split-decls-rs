macro_rules! MH_BINDATLOAD {
    () => {
        # [doc = " the object file's undefined references are bound by the dynamic linker when loaded."] pub const MH_BINDATLOAD : u32 = 0x8 ;
    };
}

MH_BINDATLOAD!()