macro_rules! deps {
    () => {
        KNONVOLATILE_CONTEXT_POINTERS_0_0!();
        M128A!();
    };
}

macro_rules! other_137 {
    () => {
        deps!();
        # [repr (C)] # [cfg (any (target_arch = "arm64ec" , target_arch = "x86_64"))] # [derive (Clone , Copy)] pub union KNONVOLATILE_CONTEXT_POINTERS_0 { pub FloatingContext : [* mut M128A ; 16] , pub Anonymous : KNONVOLATILE_CONTEXT_POINTERS_0_0 , }
    };
}

other_137!()