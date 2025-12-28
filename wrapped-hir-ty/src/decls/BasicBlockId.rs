macro_rules! deps {
    () => {
        BasicBlock!();
    };
}

macro_rules! BasicBlockId {
    () => {
        deps!();
        pub type BasicBlockId < 'db > = Idx < BasicBlock < 'db > > ;
    };
}

BasicBlockId!();