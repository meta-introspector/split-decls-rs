macro_rules! MH_ALLMODSBOUND {
    () => {
        # [doc = " indicates that this binary binds to all two-level namespace modules of its dependent libraries. only used when MH_PREBINDABLE and MH_TWOLEVEL are both set."] pub const MH_ALLMODSBOUND : u32 = 0x1000 ;
    };
}

MH_ALLMODSBOUND!();