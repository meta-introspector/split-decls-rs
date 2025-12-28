macro_rules! deps {
    () => {
        UndirectedAdaptor!();
    };
}

macro_rules! macro_211 {
    () => {
        deps!();
        Visitable ! { delegate_impl [[G] , G , UndirectedAdaptor < G >, access0] }
    };
}

macro_211!();