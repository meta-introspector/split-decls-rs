macro_rules! deps {
    () => {
        EdgeFiltered!();
    };
}

macro_rules! macro_165 {
    () => {
        deps!();
        EdgeIndexable ! { delegate_impl [[G , F] , G , EdgeFiltered < G , F >, access0] }
    };
}

macro_165!();