macro_rules! deps {
    () => {
        Reversed!();
    };
}

macro_rules! macro_190 {
    () => {
        deps!();
        GraphProp ! { delegate_impl [[G] , G , Reversed < G >, access0] }
    };
}

macro_190!();