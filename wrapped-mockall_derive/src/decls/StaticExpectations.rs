macro_rules! deps {
    () => {
        MockFunction!();
    };
}

macro_rules! StaticExpectations {
    () => {
        deps!();
        # [doc = " An collection of Expectation's for methods returning static values"] struct StaticExpectations < 'a > { f : & 'a MockFunction }
    };
}

StaticExpectations!();