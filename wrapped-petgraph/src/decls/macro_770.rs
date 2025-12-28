macro_rules! deps {
    () => {
        Frozen!();
    };
}

macro_rules! macro_770 {
    () => {
        deps!();
        IntoEdges ! { delegate_impl [['a , 'b , G] , G , &'b Frozen <'a , G >, deref_twice] }
    };
}

macro_770!()