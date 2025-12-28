macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! VER_NDX_GLOBAL {
    () => {
        deps!();
        # [doc = " Symbol is global."] pub const VER_NDX_GLOBAL : u16 = 1 ;
    };
}

VER_NDX_GLOBAL!();