mkmod!{barrier, { 
                getname!(barrier);
                getsrc!(barrier);
                getpath!(barrier);
                get_deps!(barrier);
                get_crates!(barrier);
                mkinclude!(barrier);
                 
            }}
mkuse!{# [unstable (feature = "stdarch_arm_barrier" , issue = "117219")] pub use self :: barrier :: * ;}
mkmod!{hints, { 
                getname!(hints);
                getsrc!(hints);
                getpath!(hints);
                get_deps!(hints);
                get_crates!(hints);
                mkinclude!(hints);
                 
            }}
mkuse!{# [unstable (feature = "stdarch_arm_hints" , issue = "117218")] pub use self :: hints :: * ;}
mkmod!{neon, { 
                getname!(neon);
                getsrc!(neon);
                getpath!(neon);
                get_deps!(neon);
                get_crates!(neon);
                mkinclude!(neon);
                 
            }}
mkuse!{# [cfg (any (target_arch = "aarch64" , target_arch = "arm64ec" , target_feature = "v7" , doc))] # [cfg_attr (not (target_arch = "arm") , stable (feature = "neon_intrinsics" , since = "1.59.0"))] # [cfg_attr (target_arch = "arm" , unstable (feature = "stdarch_arm_neon_intrinsics" , issue = "111800"))] pub use self :: neon :: * ;}
mkmod!{test_support, { 
                getname!(test_support);
                getsrc!(test_support);
                getpath!(test_support);
                get_deps!(test_support);
                get_crates!(test_support);
                mkinclude!(test_support);
                 
            }}
mkmod!{sealed, { 
                getname!(sealed);
                getsrc!(sealed);
                getpath!(sealed);
                get_deps!(sealed);
                get_crates!(sealed);
                mkinclude!(sealed);
                mkitem!{mktrait!{# [unstable (feature = "stdarch_arm_barrier" , issue = "117219")] pub trait Dmb { unsafe fn __dmb (& self) ; }}}
mkitem!{mktrait!{# [unstable (feature = "stdarch_arm_barrier" , issue = "117219")] pub trait Dsb { unsafe fn __dsb (& self) ; }}}
mkitem!{mktrait!{# [unstable (feature = "stdarch_arm_barrier" , issue = "117219")] pub trait Isb { unsafe fn __isb (& self) ; }}} 
            }}