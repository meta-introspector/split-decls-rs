macro_rules! deps {
    () => {
        MockFunction!();
    };
}

macro_rules! RefRfunc {
    () => {
        deps!();
        struct RefRfunc < 'a > { f : & 'a MockFunction }
    };
}

RefRfunc!();