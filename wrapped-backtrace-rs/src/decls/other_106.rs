macro_rules! deps {
    () => {
        CONTEXT_0_0!();
        XSAVE_FORMAT!();
    };
}

macro_rules! other_106 {
    () => {
        deps!();
        # [repr (C)] # [cfg (any (target_arch = "arm64ec" , target_arch = "x86_64"))] # [derive (Clone , Copy)] pub union CONTEXT_0 { pub FltSave : XSAVE_FORMAT , pub Anonymous : CONTEXT_0_0 , }
    };
}

other_106!()