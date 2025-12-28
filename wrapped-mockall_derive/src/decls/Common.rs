macro_rules! deps {
    () => {
        MockFunction!();
    };
}

macro_rules! Common {
    () => {
        deps!();
        # [doc = " Holds parts of the expectation that are common for all output types"] struct Common < 'a > { f : & 'a MockFunction }
    };
}

Common!()