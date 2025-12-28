macro_rules! deps {
    () => {
        UndirectedAdaptor!();
    };
}

macro_rules! macro_212 {
    () => {
        deps!();
        NodeIndexable ! { delegate_impl [[G] , G , UndirectedAdaptor < G >, access0] }
    };
}

macro_212!()