macro_rules! deps {
    () => {
        MockFunction!();
    };
}

macro_rules! GenericExpectations {
    () => {
        deps!();
        struct GenericExpectations < 'a > { f : & 'a MockFunction }
    };
}

GenericExpectations!();