macro_rules! deps {
    () => {
        Frozen!();
    };
}

macro_rules! macro_771 {
    () => {
        deps!();
        IntoEdgesDirected ! { delegate_impl [['a , 'b , G] , G , &'b Frozen <'a , G >, deref_twice] }
    };
}

macro_771!();