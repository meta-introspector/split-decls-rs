macro_rules! PT_GNU_RELRO {
    () => {
        pub const PT_GNU_RELRO : u32 = 0x6474_e552 ;
    };
}

PT_GNU_RELRO!();