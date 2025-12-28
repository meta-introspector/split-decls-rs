macro_rules! deps {
    () => {
        Frozen!();
    };
}

macro_rules! macro_772 {
    () => {
        deps!();
        IntoNeighbors ! { delegate_impl [['a , 'b , G] , G , &'b Frozen <'a , G >, deref_twice] }
    };
}

macro_772!()