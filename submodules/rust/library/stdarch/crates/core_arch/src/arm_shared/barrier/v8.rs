mkitem!{mkstruct!{# [doc = " Full system is the required shareability domain, reads are the required"] # [doc = " access type"] # [unstable (feature = "stdarch_arm_barrier" , issue = "117219")] pub struct LD ;}}
mkitem!{dmb_dsb ! (LD) ;}
mkitem!{mkstruct!{# [doc = " Inner Shareable is the required shareability domain, reads are the required"] # [doc = " access type"] # [unstable (feature = "stdarch_arm_barrier" , issue = "117219")] pub struct ISHLD ;}}
mkitem!{dmb_dsb ! (ISHLD) ;}
mkitem!{mkstruct!{# [doc = " Non-shareable is the required shareability domain, reads are the required"] # [doc = " access type"] # [unstable (feature = "stdarch_arm_barrier" , issue = "117219")] pub struct NSHLD ;}}
mkitem!{dmb_dsb ! (NSHLD) ;}
mkitem!{mkstruct!{# [doc = " Outer Shareable is the required shareability domain, reads are the required"] # [doc = " access type"] # [unstable (feature = "stdarch_arm_barrier" , issue = "117219")] pub struct OSHLD ;}}
mkitem!{dmb_dsb ! (OSHLD) ;}