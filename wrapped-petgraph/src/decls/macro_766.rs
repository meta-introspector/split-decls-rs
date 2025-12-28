macro_rules! deps {
    () => {
        Frozen!();
    };
}

macro_rules! macro_766 {
    () => {
        deps!();
        DataMap ! { delegate_impl [['a , G] , G , Frozen <'a , G >, deref_twice] }
    };
}

macro_766!()