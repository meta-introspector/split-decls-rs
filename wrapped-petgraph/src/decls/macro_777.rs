macro_rules! deps {
    () => {
        Frozen!();
    };
}

macro_rules! macro_777 {
    () => {
        deps!();
        NodeCount ! { delegate_impl [['a , G] , G , Frozen <'a , G >, deref_twice] }
    };
}

macro_777!();