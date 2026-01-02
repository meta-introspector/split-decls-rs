mkmod!{sat, { 
                getname!(sat);
                getsrc!(sat);
                getpath!(sat);
                get_deps!(sat);
                get_crates!(sat);
                mkinclude!(sat);
                 
            }}
mkuse!{# [cfg (any (target_feature = "v6" , doc))] # [unstable (feature = "stdarch_arm_sat" , issue = "none")] pub use self :: sat :: * ;}
mkmod!{dsp, { 
                getname!(dsp);
                getsrc!(dsp);
                getpath!(dsp);
                get_deps!(dsp);
                get_crates!(dsp);
                mkinclude!(dsp);
                 
            }}
mkuse!{# [cfg (any (all (target_feature = "v5te" , not (target_feature = "mclass")) , all (target_feature = "mclass" , target_feature = "dsp") , doc ,))] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub use self :: dsp :: * ;}
mkmod!{simd32, { 
                getname!(simd32);
                getsrc!(simd32);
                getpath!(simd32);
                get_deps!(simd32);
                get_crates!(simd32);
                mkinclude!(simd32);
                 
            }}
mkuse!{# [cfg (any (all (target_feature = "v6" , not (target_feature = "mclass")) , all (target_feature = "mclass" , target_feature = "dsp") , doc ,))] # [unstable (feature = "stdarch_arm_dsp" , issue = "117237")] pub use self :: simd32 :: * ;}
mkuse!{# [unstable (feature = "stdarch_arm_neon_intrinsics" , issue = "111800")] pub use crate :: core_arch :: arm_shared :: * ;}
mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}