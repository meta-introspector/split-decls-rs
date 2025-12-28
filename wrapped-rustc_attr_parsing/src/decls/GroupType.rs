macro_rules! deps {
    () => {
        GroupTypeInner!();
    };
}

macro_rules! GroupType {
    () => {
        deps!();
        type GroupType < S > = LazyLock < GroupTypeInner < S > > ;
    };
}

GroupType!();