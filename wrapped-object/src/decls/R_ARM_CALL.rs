macro_rules! R_ARM_CALL {
    () => {
        # [doc = " PC relative 24 bit (`BL`, `BLX`)."] pub const R_ARM_CALL : u32 = 28 ;
    };
}

R_ARM_CALL!();