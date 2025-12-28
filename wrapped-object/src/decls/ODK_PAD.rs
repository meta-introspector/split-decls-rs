macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! ODK_PAD {
    () => {
        deps!();
        # [doc = " Section padding options."] pub const ODK_PAD : u32 = 3 ;
    };
}

ODK_PAD!()