macro_rules! deps {
    () => {
        Frozen!();
    };
}

macro_rules! macro_781 {
    () => {
        deps!();
        GraphProp ! { delegate_impl [['a , G] , G , Frozen <'a , G >, deref_twice] }
    };
}

macro_781!()