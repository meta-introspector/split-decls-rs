macro_rules! deps {
    () => {
        M128A!();
    };
}

macro_rules! KNONVOLATILE_CONTEXT_POINTERS_0_0 {
    () => {
        deps!();
        # [repr (C)] # [cfg (any (target_arch = "arm64ec" , target_arch = "x86_64"))] # [derive (Clone , Copy)] pub struct KNONVOLATILE_CONTEXT_POINTERS_0_0 { pub Xmm0 : * mut M128A , pub Xmm1 : * mut M128A , pub Xmm2 : * mut M128A , pub Xmm3 : * mut M128A , pub Xmm4 : * mut M128A , pub Xmm5 : * mut M128A , pub Xmm6 : * mut M128A , pub Xmm7 : * mut M128A , pub Xmm8 : * mut M128A , pub Xmm9 : * mut M128A , pub Xmm10 : * mut M128A , pub Xmm11 : * mut M128A , pub Xmm12 : * mut M128A , pub Xmm13 : * mut M128A , pub Xmm14 : * mut M128A , pub Xmm15 : * mut M128A , }
    };
}

KNONVOLATILE_CONTEXT_POINTERS_0_0!()