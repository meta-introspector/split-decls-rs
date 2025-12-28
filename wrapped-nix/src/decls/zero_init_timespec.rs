macro_rules! zero_init_timespec {
    () => {
        const fn zero_init_timespec () -> timespec { unsafe { std :: mem :: transmute ([0u8 ; std :: mem :: size_of :: < timespec > ()]) } }
    };
}

zero_init_timespec!()