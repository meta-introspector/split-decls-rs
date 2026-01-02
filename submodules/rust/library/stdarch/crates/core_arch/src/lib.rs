mkitem!{# [cfg (test)] # [macro_use] extern crate std ;}
mkmod!{core_arch, { 
                getname!(core_arch);
                getsrc!(core_arch);
                getpath!(core_arch);
                get_deps!(core_arch);
                get_crates!(core_arch);
                mkinclude!(core_arch);
                 
            }}
mkmod!{arch, { 
                getname!(arch);
                getsrc!(arch);
                getpath!(arch);
                get_deps!(arch);
                get_crates!(arch);
                mkinclude!(arch);
                mkuse!{# [stable (feature = "stdsimd" , since = "1.27.0")] # [allow (unused_imports)] pub use crate :: core_arch :: arch :: * ;}
mkuse!{# [stable (feature = "stdsimd" , since = "1.27.0")] pub use core :: arch :: asm ;} 
            }}
mkuse!{# [allow (unused_imports)] use core :: { array , convert , ffi , fmt , hint , intrinsics , marker , mem , ops , ptr , sync } ;}