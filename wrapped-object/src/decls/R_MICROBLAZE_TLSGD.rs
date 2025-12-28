macro_rules! deps {
    () => {
        Dynamic!();
    };
}

macro_rules! R_MICROBLAZE_TLSGD {
    () => {
        deps!();
        # [doc = " TLS General Dynamic."] pub const R_MICROBLAZE_TLSGD : u32 = 23 ;
    };
}

R_MICROBLAZE_TLSGD!();