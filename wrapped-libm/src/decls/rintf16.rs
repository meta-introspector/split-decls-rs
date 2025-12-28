macro_rules! deps {
    () => {
        Round!();
    };
}

macro_rules! rintf16 {
    () => {
        deps!();
        # [doc = " Round `x` to the nearest integer, breaking ties toward even."] # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn rintf16 (x : f16) -> f16 { select_implementation ! { name : rintf16 , use_arch : all (target_arch = "aarch64" , target_feature = "fp16") , args : x , } super :: generic :: rint_round (x , Round :: Nearest) . val }
    };
}

rintf16!();