macro_rules! deps {
    () => {
        EdgeFiltered!();
    };
}

macro_rules! macro_160 {
    () => {
        deps!();
        IntoNodeIdentifiers ! { delegate_impl [['a , G , F] , G , &'a EdgeFiltered < G , F >, access0] }
    };
}

macro_160!()