macro_rules! deps {
    () => {
        MockFunction!();
    };
}

macro_rules! StaticExpectation {
    () => {
        deps!();
        # [doc = " An expectation type for functions return a `'static` value"] struct StaticExpectation < 'a > { f : & 'a MockFunction }
    };
}

StaticExpectation!()