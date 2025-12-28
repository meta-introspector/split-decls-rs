macro_rules! deps {
    () => {
        Frozen!();
    };
}

macro_rules! macro_779 {
    () => {
        deps!();
        EdgeCount ! { delegate_impl [['a , G] , G , Frozen <'a , G >, deref_twice] }
    };
}

macro_779!();