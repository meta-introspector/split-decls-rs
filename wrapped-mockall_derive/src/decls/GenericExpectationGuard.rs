macro_rules! deps {
    () => {
        MockFunction!();
    };
}

macro_rules! GenericExpectationGuard {
    () => {
        deps!();
        # [doc = " The ExpectationGuard structure for static methods with generic types"] struct GenericExpectationGuard < 'a > { f : & 'a MockFunction }
    };
}

GenericExpectationGuard!()