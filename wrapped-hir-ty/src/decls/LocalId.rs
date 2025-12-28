macro_rules! deps {
    () => {
        Local!();
    };
}

macro_rules! LocalId {
    () => {
        deps!();
        pub type LocalId < 'db > = Idx < Local < 'db > > ;
    };
}

LocalId!();