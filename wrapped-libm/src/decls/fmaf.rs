macro_rules! deps {
    () => {
        Round!();
    };
}

macro_rules! fmaf {
    () => {
        deps!();
        # [doc = " Floating multiply add (f32)"] # [doc = ""] # [doc = " Computes `(x*y)+z`, rounded as one ternary operation (i.e. calculated with infinite precision)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fmaf (x : f32 , y : f32 , z : f32) -> f32 { select_implementation ! { name : fmaf , use_arch : any (all (target_arch = "aarch64" , target_feature = "neon") , target_feature = "sse2" ,) , args : x , y , z , } generic :: fma_wide_round (x , y , z , Round :: Nearest) . val }
    };
}

fmaf!();