macro_rules! deps {
    () => {
        MockFunction!();
    };
}

macro_rules! RefMutRfunc {
    () => {
        deps!();
        struct RefMutRfunc < 'a > { f : & 'a MockFunction }
    };
}

RefMutRfunc!();