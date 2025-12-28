macro_rules! deps {
    () => {
        NodeFiltered!();
    };
}

macro_rules! macro_140 {
    () => {
        deps!();
        GraphProp ! { delegate_impl [[G , F] , G , NodeFiltered < G , F >, access0] }
    };
}

macro_140!()