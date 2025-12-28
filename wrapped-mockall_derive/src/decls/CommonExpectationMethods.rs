macro_rules! deps {
    () => {
        MockFunction!();
    };
}

macro_rules! CommonExpectationMethods {
    () => {
        deps!();
        # [doc = " Generates methods that are common for all Expectation types"] struct CommonExpectationMethods < 'a > { f : & 'a MockFunction }
    };
}

CommonExpectationMethods!()