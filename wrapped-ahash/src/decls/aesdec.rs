macro_rules! aesdec {
    () => {
        # [cfg (any (all (target_arch = "aarch64" , target_feature = "aes" , not (miri)) , all (feature = "nightly-arm-aes" , target_arch = "arm" , target_feature = "aes" , not (miri)) ,))] # [allow (unused)] # [inline (always)] pub (crate) fn aesdec (value : u128 , xor : u128) -> u128 { # [cfg (target_arch = "aarch64")] use core :: arch :: aarch64 :: * ; # [cfg (target_arch = "arm")] use core :: arch :: arm :: * ; let res = unsafe { vaesimcq_u8 (vaesdq_u8 (transmute ! (value) , transmute ! (0u128))) } ; let value : u128 = transmute ! (res) ; xor ^ value }
    };
}

aesdec!();