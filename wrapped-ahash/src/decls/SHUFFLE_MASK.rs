macro_rules! SHUFFLE_MASK {
    () => {
        # [doc = " This is a constant with a lot of special properties found by automated search."] # [doc = " See the unit tests below. (Below are alternative values)"] # [cfg (all (target_feature = "ssse3" , not (miri)))] const SHUFFLE_MASK : u128 = 0x020a0700_0c01030e_050f0d08_06090b04_u128 ;
    };
}

SHUFFLE_MASK!();