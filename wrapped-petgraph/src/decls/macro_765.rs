macro_rules! deps {
    () => {
        Frozen!();
    };
}

macro_rules! macro_765 {
    () => {
        deps!();
        Data ! { delegate_impl [['a , G] , G , Frozen <'a , G >, deref_twice] }
    };
}

macro_765!();