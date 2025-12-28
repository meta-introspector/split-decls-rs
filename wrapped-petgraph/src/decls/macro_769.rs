macro_rules! deps {
    () => {
        Frozen!();
    };
}

macro_rules! macro_769 {
    () => {
        deps!();
        IntoEdgeReferences ! { delegate_impl [['a , 'b , G] , G , &'b Frozen <'a , G >, deref_twice] }
    };
}

macro_769!();