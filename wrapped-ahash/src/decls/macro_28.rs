macro_rules! deps {
    () => {
        AHasher!();
    };
}

macro_rules! macro_28 {
    () => {
        deps!();
        cfg_if :: cfg_if ! { if # [cfg (any (all (any (target_arch = "x86" , target_arch = "x86_64") , target_feature = "aes" , not (miri)) , all (target_arch = "aarch64" , target_feature = "aes" , not (miri)) , all (feature = "nightly-arm-aes" , target_arch = "arm" , target_feature = "aes" , not (miri)) ,))] { mod aes_hash ; pub use crate :: aes_hash :: AHasher ; } else { pub use crate :: fallback_hash :: AHasher ; } }
    };
}

macro_28!();