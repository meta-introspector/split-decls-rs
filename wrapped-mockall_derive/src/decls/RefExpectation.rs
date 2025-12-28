macro_rules! deps {
    () => {
        MockFunction!();
    };
}

macro_rules! RefExpectation {
    () => {
        deps!();
        # [doc = " An expectation type for functions that take a &self and return a reference"] struct RefExpectation < 'a > { f : & 'a MockFunction }
    };
}

RefExpectation!()