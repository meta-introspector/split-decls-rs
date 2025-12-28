macro_rules! deps {
    () => {
        AnyExpectations!();
    };
}

macro_rules! macro_8 {
    () => {
        deps!();
        downcast ! (dyn AnyExpectations) ;
    };
}

macro_8!();