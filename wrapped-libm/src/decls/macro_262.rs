macro_rules! macro_262 {
    () => {
        cfg_if ! { if # [cfg (all (target_arch = "x86" , not (target_feature = "sse2")))] { mod i586 ; pub use i586 :: { ceil , floor } ; } }
    };
}

macro_262!();