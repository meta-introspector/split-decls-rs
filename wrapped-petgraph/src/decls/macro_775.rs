macro_rules! deps {
    () => {
        Frozen!();
    };
}

macro_rules! macro_775 {
    () => {
        deps!();
        IntoNodeReferences ! { delegate_impl [['a , 'b , G] , G , &'b Frozen <'a , G >, deref_twice] }
    };
}

macro_775!();