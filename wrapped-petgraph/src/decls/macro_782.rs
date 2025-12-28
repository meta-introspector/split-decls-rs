macro_rules! deps {
    () => {
        Frozen!();
    };
}

macro_rules! macro_782 {
    () => {
        deps!();
        Visitable ! { delegate_impl [['a , G] , G , Frozen <'a , G >, deref_twice] }
    };
}

macro_782!()