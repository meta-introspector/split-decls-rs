macro_rules! deps {
    () => {
        ErrorPositions!();
        ErrorPositionsInner!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl ErrorPositions { fn new_0 () -> Self { Self (ErrorPositionsInner :: None) } fn new_1 (a : Pos) -> Self { Self (ErrorPositionsInner :: One (a)) } fn new_2 (a : Pos , b : Pos) -> Self { Self (ErrorPositionsInner :: Two (a , b)) } }
    };
}

impl_10!()