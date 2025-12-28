macro_rules! deps {
    () => {
        AnyExpectations!();
    };
}

macro_rules! macro_3 {
    () => {
        deps!();
        downcast ! (dyn AnyExpectations) ;
    };
}

macro_3!()