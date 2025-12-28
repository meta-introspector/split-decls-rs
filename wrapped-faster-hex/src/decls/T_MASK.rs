macro_rules! T_MASK {
    () => {
        # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] const T_MASK : i32 = 65535 ;
    };
}

T_MASK!()