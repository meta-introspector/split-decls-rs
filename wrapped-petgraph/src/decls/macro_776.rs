macro_rules! deps {
    () => {
        Frozen!();
    };
}

macro_rules! macro_776 {
    () => {
        deps!();
        NodeCompactIndexable ! { delegate_impl [['a , G] , G , Frozen <'a , G >, deref_twice] }
    };
}

macro_776!()