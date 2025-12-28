macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! VERSYM_VERSION {
    () => {
        deps!();
        # [doc = " Symbol version index."] pub const VERSYM_VERSION : u16 = 0x7fff ;
    };
}

VERSYM_VERSION!()