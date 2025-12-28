macro_rules! deps {
    () => {
        MockFunction!();
        GenericExpectations!();
    };
}

macro_rules! StaticGenericExpectations {
    () => {
        deps!();
        # [doc = " Generates methods for GenericExpectations for methods returning static"] # [doc = " values"] struct StaticGenericExpectations < 'a > { f : & 'a MockFunction }
    };
}

StaticGenericExpectations!();