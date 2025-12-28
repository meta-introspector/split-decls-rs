macro_rules! deps {
    () => {
        Version!();
    };
}

macro_rules! VER_FLG_BASE {
    () => {
        deps!();
        # [doc = " Version definition of file itself"] pub const VER_FLG_BASE : u16 = 0x1 ;
    };
}

VER_FLG_BASE!();