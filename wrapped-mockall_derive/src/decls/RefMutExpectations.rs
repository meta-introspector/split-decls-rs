macro_rules! deps {
    () => {
        MockFunction!();
    };
}

macro_rules! RefMutExpectations {
    () => {
        deps!();
        # [doc = " An collection of RefMutExpectation's"] struct RefMutExpectations < 'a > { f : & 'a MockFunction }
    };
}

RefMutExpectations!();