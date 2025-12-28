macro_rules! deps {
    () => {
        MockFunction!();
    };
}

macro_rules! Methods {
    () => {
        deps!();
        # [doc = " A collection of methods defined in one spot"] struct Methods (Vec < MockFunction >) ;
    };
}

Methods!();