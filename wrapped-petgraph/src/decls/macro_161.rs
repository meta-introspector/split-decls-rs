macro_rules! deps {
    () => {
        EdgeFiltered!();
    };
}

macro_rules! macro_161 {
    () => {
        deps!();
        IntoNodeReferences ! { delegate_impl [['a , G , F] , G , &'a EdgeFiltered < G , F >, access0] }
    };
}

macro_161!();