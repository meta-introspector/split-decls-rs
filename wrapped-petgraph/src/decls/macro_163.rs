macro_rules! deps {
    () => {
        EdgeFiltered!();
    };
}

macro_rules! macro_163 {
    () => {
        deps!();
        NodeCount ! { delegate_impl [[G , F] , G , EdgeFiltered < G , F >, access0] }
    };
}

macro_163!()