macro_rules! deps {
    () => {
        MockFunction!();
    };
}

macro_rules! ConcreteExpectationGuard {
    () => {
        deps!();
        # [doc = " The ExpectationGuard structure for static methods with no generic types"] struct ConcreteExpectationGuard < 'a > { f : & 'a MockFunction }
    };
}

ConcreteExpectationGuard!();