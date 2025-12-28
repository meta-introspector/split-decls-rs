macro_rules! deps {
    () => {
        CustomFormat!();
    };
}

macro_rules! RFC2822 {
    () => {
        deps!();
        # [doc = " E.g. `Thu, 18 Aug 2022 12:45:06 +0800`"] pub const RFC2822 : CustomFormat = CustomFormat ("%a, %d %b %Y %H:%M:%S %z") ;
    };
}

RFC2822!()