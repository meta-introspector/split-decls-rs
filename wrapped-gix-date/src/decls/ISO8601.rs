macro_rules! deps {
    () => {
        CustomFormat!();
    };
}

macro_rules! ISO8601 {
    () => {
        deps!();
        # [doc = " E.g. `2022-08-17 22:04:58 +0200`"] pub const ISO8601 : CustomFormat = CustomFormat ("%Y-%m-%d %H:%M:%S %z") ;
    };
}

ISO8601!();