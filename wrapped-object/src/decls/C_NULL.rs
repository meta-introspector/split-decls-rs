macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! C_NULL {
    () => {
        deps!();
        # [doc = " Symbol table entry marked for deletion."] pub const C_NULL : u8 = 0 ;
    };
}

C_NULL!()