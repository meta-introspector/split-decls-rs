macro_rules! R_X86_64_REX_GOTPCRELX {
    () => {
        # [doc = " Load from 32 bit signed pc relative offset to GOT entry with REX prefix, relaxable."] pub const R_X86_64_REX_GOTPCRELX : u32 = 42 ;
    };
}

R_X86_64_REX_GOTPCRELX!();