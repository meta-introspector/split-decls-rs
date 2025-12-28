macro_rules! deps {
    () => {
        Architecture!();
    };
}

macro_rules! EM_XTENSA {
    () => {
        deps!();
        # [doc = " Tensilica Xtensa Architecture"] pub const EM_XTENSA : u16 = 94 ;
    };
}

EM_XTENSA!()