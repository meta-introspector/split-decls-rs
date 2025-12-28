macro_rules! deps {
    () => {
        MockFunction!();
    };
}

macro_rules! RefExpectations {
    () => {
        deps!();
        # [doc = " An collection of RefExpectation's"] struct RefExpectations < 'a > { f : & 'a MockFunction }
    };
}

RefExpectations!()