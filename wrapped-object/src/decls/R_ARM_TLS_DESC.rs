macro_rules! deps {
    () => {
        Dynamic!();
    };
}

macro_rules! R_ARM_TLS_DESC {
    () => {
        deps!();
        # [doc = " Dynamic relocation."] pub const R_ARM_TLS_DESC : u32 = 13 ;
    };
}

R_ARM_TLS_DESC!();