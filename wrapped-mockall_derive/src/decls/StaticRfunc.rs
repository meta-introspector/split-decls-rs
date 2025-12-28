macro_rules! deps {
    () => {
        MockFunction!();
    };
}

macro_rules! StaticRfunc {
    () => {
        deps!();
        struct StaticRfunc < 'a > { f : & 'a MockFunction }
    };
}

StaticRfunc!()