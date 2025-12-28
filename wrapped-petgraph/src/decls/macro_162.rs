macro_rules! deps {
    () => {
        EdgeFiltered!();
    };
}

macro_rules! macro_162 {
    () => {
        deps!();
        NodeCompactIndexable ! { delegate_impl [[G , F] , G , EdgeFiltered < G , F >, access0] }
    };
}

macro_162!();