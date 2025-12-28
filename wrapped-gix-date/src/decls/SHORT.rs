macro_rules! deps {
    () => {
        CustomFormat!();
    };
}

macro_rules! SHORT {
    () => {
        deps!();
        # [doc = " E.g. `2018-12-24`"] pub const SHORT : CustomFormat = CustomFormat ("%Y-%m-%d") ;
    };
}

SHORT!()