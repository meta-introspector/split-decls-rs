macro_rules! deps {
    () => {
        CustomFormat!();
    };
}

macro_rules! ISO8601_STRICT {
    () => {
        deps!();
        # [doc = " E.g. `2022-08-17T21:43:13+08:00`"] pub const ISO8601_STRICT : CustomFormat = CustomFormat ("%Y-%m-%dT%H:%M:%S%:z") ;
    };
}

ISO8601_STRICT!()