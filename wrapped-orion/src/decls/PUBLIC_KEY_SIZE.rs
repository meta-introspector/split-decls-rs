macro_rules! deps {
    () => {
        EncapsulationKey!();
    };
}

macro_rules! PUBLIC_KEY_SIZE {
    () => {
        deps!();
        # [doc = " Size of public [EncapsulationKey]."] pub const PUBLIC_KEY_SIZE : usize = 1216 ;
    };
}

PUBLIC_KEY_SIZE!();