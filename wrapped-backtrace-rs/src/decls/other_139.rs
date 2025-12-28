macro_rules! deps {
    () => {
        KNONVOLATILE_CONTEXT_POINTERS_1_0!();
    };
}

macro_rules! other_139 {
    () => {
        deps!();
        # [repr (C)] # [cfg (any (target_arch = "arm64ec" , target_arch = "x86_64"))] # [derive (Clone , Copy)] pub union KNONVOLATILE_CONTEXT_POINTERS_1 { pub IntegerContext : [* mut u64 ; 16] , pub Anonymous : KNONVOLATILE_CONTEXT_POINTERS_1_0 , }
    };
}

other_139!();