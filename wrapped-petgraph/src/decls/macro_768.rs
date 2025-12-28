macro_rules! deps {
    () => {
        Frozen!();
    };
}

macro_rules! macro_768 {
    () => {
        deps!();
        GetAdjacencyMatrix ! { delegate_impl [['a , G] , G , Frozen <'a , G >, deref_twice] }
    };
}

macro_768!();