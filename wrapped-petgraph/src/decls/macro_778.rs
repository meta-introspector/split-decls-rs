macro_rules! deps {
    () => {
        Frozen!();
    };
}

macro_rules! macro_778 {
    () => {
        deps!();
        NodeIndexable ! { delegate_impl [['a , G] , G , Frozen <'a , G >, deref_twice] }
    };
}

macro_778!();