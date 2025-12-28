macro_rules! deps {
    () => {
        CLSCTX!();
    };
}

macro_rules! CLSCTX_ALL {
    () => {
        deps!();
        pub const CLSCTX_ALL : CLSCTX = 23u32 ;
    };
}

CLSCTX_ALL!()