macro_rules! deps {
    () => {
        Frozen!();
    };
}

macro_rules! macro_780 {
    () => {
        deps!();
        EdgeIndexable ! { delegate_impl [['a , G] , G , Frozen <'a , G >, deref_twice] }
    };
}

macro_780!()