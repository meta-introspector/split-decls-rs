macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! VER_NDX_LOCAL {
    () => {
        deps!();
        # [doc = " Symbol is local."] pub const VER_NDX_LOCAL : u16 = 0 ;
    };
}

VER_NDX_LOCAL!();