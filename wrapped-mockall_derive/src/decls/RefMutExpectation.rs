macro_rules! deps {
    () => {
        MockFunction!();
    };
}

macro_rules! RefMutExpectation {
    () => {
        deps!();
        # [doc = " For methods that take &mut self and return a reference"] struct RefMutExpectation < 'a > { f : & 'a MockFunction }
    };
}

RefMutExpectation!()