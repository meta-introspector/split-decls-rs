macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! R_E2K_32_DYNOPT {
    () => {
        deps!();
        # [doc = " Symbol value if resolved by the definition in the same"] # [doc = " compilation unit or NULL otherwise, 32-bit case."] pub const R_E2K_32_DYNOPT : u32 = 13 ;
    };
}

R_E2K_32_DYNOPT!()