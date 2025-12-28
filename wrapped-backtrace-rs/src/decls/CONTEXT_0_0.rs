macro_rules! deps {
    () => {
        M128A!();
    };
}

macro_rules! CONTEXT_0_0 {
    () => {
        deps!();
        # [repr (C)] # [cfg (any (target_arch = "arm64ec" , target_arch = "x86_64"))] # [derive (Clone , Copy)] pub struct CONTEXT_0_0 { pub Header : [M128A ; 2] , pub Legacy : [M128A ; 8] , pub Xmm0 : M128A , pub Xmm1 : M128A , pub Xmm2 : M128A , pub Xmm3 : M128A , pub Xmm4 : M128A , pub Xmm5 : M128A , pub Xmm6 : M128A , pub Xmm7 : M128A , pub Xmm8 : M128A , pub Xmm9 : M128A , pub Xmm10 : M128A , pub Xmm11 : M128A , pub Xmm12 : M128A , pub Xmm13 : M128A , pub Xmm14 : M128A , pub Xmm15 : M128A , }
    };
}

CONTEXT_0_0!()