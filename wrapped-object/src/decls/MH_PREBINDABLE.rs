macro_rules! MH_PREBINDABLE {
    () => {
        # [doc = " the binary is not prebound but can have its prebinding redone. only used when MH_PREBOUND is not set."] pub const MH_PREBINDABLE : u32 = 0x800 ;
    };
}

MH_PREBINDABLE!();