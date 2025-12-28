macro_rules! deps {
    () => {
        Frozen!();
    };
}

macro_rules! macro_773 {
    () => {
        deps!();
        IntoNeighborsDirected ! { delegate_impl [['a , 'b , G] , G , &'b Frozen <'a , G >, deref_twice] }
    };
}

macro_773!();