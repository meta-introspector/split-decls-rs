macro_rules! deps {
    () => {
        Dynamic!();
    };
}

macro_rules! R_MICROBLAZE_TLSLD {
    () => {
        deps!();
        # [doc = " TLS Local Dynamic."] pub const R_MICROBLAZE_TLSLD : u32 = 24 ;
    };
}

R_MICROBLAZE_TLSLD!();