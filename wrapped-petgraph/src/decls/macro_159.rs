macro_rules! deps {
    () => {
        EdgeFiltered!();
    };
}

macro_rules! macro_159 {
    () => {
        deps!();
        GraphProp ! { delegate_impl [[G , F] , G , EdgeFiltered < G , F >, access0] }
    };
}

macro_159!()