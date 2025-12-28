macro_rules! deps {
    () => {
        UndirectedAdaptor!();
    };
}

macro_rules! macro_210 {
    () => {
        deps!();
        Data ! { delegate_impl [[G] , G , UndirectedAdaptor < G >, access0] }
    };
}

macro_210!();