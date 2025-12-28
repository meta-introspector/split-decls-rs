macro_rules! deps {
    () => {
        Frozen!();
    };
}

macro_rules! macro_774 {
    () => {
        deps!();
        IntoNodeIdentifiers ! { delegate_impl [['a , 'b , G] , G , &'b Frozen <'a , G >, deref_twice] }
    };
}

macro_774!();