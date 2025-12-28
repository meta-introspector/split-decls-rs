macro_rules! deps {
    () => {
        UndirectedAdaptor!();
    };
}

macro_rules! macro_213 {
    () => {
        deps!();
        NodeCompactIndexable ! { delegate_impl [[G] , G , UndirectedAdaptor < G >, access0] }
    };
}

macro_213!();