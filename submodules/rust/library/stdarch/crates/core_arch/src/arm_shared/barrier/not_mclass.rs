mkitem!{mkstruct!{# [doc = " Full system is the required shareability domain, writes are the required"] # [doc = " access type"] # [unstable (feature = "stdarch_arm_barrier" , issue = "117219")] pub struct ST ;}}
mkitem!{dmb_dsb ! (ST) ;}
mkitem!{mkstruct!{# [doc = " Inner Shareable is the required shareability domain, reads and writes are"] # [doc = " the required access types"] # [unstable (feature = "stdarch_arm_barrier" , issue = "117219")] pub struct ISH ;}}
mkitem!{dmb_dsb ! (ISH) ;}
mkitem!{mkstruct!{# [doc = " Inner Shareable is the required shareability domain, writes are the required"] # [doc = " access type"] # [unstable (feature = "stdarch_arm_barrier" , issue = "117219")] pub struct ISHST ;}}
mkitem!{dmb_dsb ! (ISHST) ;}
mkitem!{mkstruct!{# [doc = " Non-shareable is the required shareability domain, reads and writes are the"] # [doc = " required access types"] # [unstable (feature = "stdarch_arm_barrier" , issue = "117219")] pub struct NSH ;}}
mkitem!{dmb_dsb ! (NSH) ;}
mkitem!{mkstruct!{# [doc = " Non-shareable is the required shareability domain, writes are the required"] # [doc = " access type"] # [unstable (feature = "stdarch_arm_barrier" , issue = "117219")] pub struct NSHST ;}}
mkitem!{dmb_dsb ! (NSHST) ;}
mkitem!{mkstruct!{# [doc = " Outer Shareable is the required shareability domain, reads and writes are"] # [doc = " the required access types"] # [unstable (feature = "stdarch_arm_barrier" , issue = "117219")] pub struct OSH ;}}
mkitem!{dmb_dsb ! (OSH) ;}
mkitem!{mkstruct!{# [doc = " Outer Shareable is the required shareability domain, writes are the required"] # [doc = " access type"] # [unstable (feature = "stdarch_arm_barrier" , issue = "117219")] pub struct OSHST ;}}
mkitem!{dmb_dsb ! (OSHST) ;}