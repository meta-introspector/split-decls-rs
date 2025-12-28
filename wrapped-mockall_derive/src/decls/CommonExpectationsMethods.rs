macro_rules! deps {
    () => {
        MockFunction!();
    };
}

macro_rules! CommonExpectationsMethods {
    () => {
        deps!();
        # [doc = " Holds the moethods of the Expectations object that are common for all"] # [doc = " Expectation types"] struct CommonExpectationsMethods < 'a > { f : & 'a MockFunction }
    };
}

CommonExpectationsMethods!();