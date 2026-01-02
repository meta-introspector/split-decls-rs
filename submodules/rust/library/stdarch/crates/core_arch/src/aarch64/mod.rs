mkmod!{mte, { 
                getname!(mte);
                getsrc!(mte);
                getpath!(mte);
                get_deps!(mte);
                get_crates!(mte);
                mkinclude!(mte);
                 
            }}
mkuse!{# [unstable (feature = "stdarch_aarch64_mte" , issue = "129010")] pub use self :: mte :: * ;}
mkmod!{neon, { 
                getname!(neon);
                getsrc!(neon);
                getpath!(neon);
                get_deps!(neon);
                get_crates!(neon);
                mkinclude!(neon);
                 
            }}
mkuse!{# [stable (feature = "neon_intrinsics" , since = "1.59.0")] pub use self :: neon :: * ;}
mkmod!{tme, { 
                getname!(tme);
                getsrc!(tme);
                getpath!(tme);
                get_deps!(tme);
                get_crates!(tme);
                mkinclude!(tme);
                 
            }}
mkuse!{# [unstable (feature = "stdarch_aarch64_tme" , issue = "117216")] pub use self :: tme :: * ;}
mkmod!{prefetch, { 
                getname!(prefetch);
                getsrc!(prefetch);
                getpath!(prefetch);
                get_deps!(prefetch);
                get_crates!(prefetch);
                mkinclude!(prefetch);
                 
            }}
mkuse!{# [unstable (feature = "stdarch_aarch64_prefetch" , issue = "117217")] pub use self :: prefetch :: * ;}
mkuse!{# [stable (feature = "neon_intrinsics" , since = "1.59.0")] pub use super :: arm_shared :: * ;}
mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}
mkmod!{test_support, { 
                getname!(test_support);
                getsrc!(test_support);
                getpath!(test_support);
                get_deps!(test_support);
                get_crates!(test_support);
                mkinclude!(test_support);
                 
            }}