macro_rules! deps {
    () => {
        UndirectedAdaptor!();
    };
}

macro_rules! macro_209 {
    () => {
        deps!();
        GraphBase ! { delegate_impl [[G] , G , UndirectedAdaptor < G >, access0] }
    };
}

macro_209!()