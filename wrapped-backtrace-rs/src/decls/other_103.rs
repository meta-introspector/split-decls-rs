macro_rules! deps {
    () => {
        CONTEXT_0_0!();
    };
}

macro_rules! other_103 {
    () => {
        deps!();
        # [repr (C)] # [cfg (target_arch = "aarch64")] # [derive (Clone , Copy)] pub union CONTEXT_0 { pub Anonymous : CONTEXT_0_0 , pub X : [u64 ; 31] , }
    };
}

other_103!();