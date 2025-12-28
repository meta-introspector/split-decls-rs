macro_rules! deps {
    () => {
        MockFunction!();
    };
}

macro_rules! ExpectationGuardCommonMethods {
    () => {
        deps!();
        # [doc = " The ExpectationGuard structure for static methods with no generic types"] struct ExpectationGuardCommonMethods < 'a > { f : & 'a MockFunction }
    };
}

ExpectationGuardCommonMethods!()