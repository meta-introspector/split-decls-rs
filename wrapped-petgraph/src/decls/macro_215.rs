macro_rules! deps {
    () => {
        UndirectedAdaptor!();
    };
}

macro_rules! macro_215 {
    () => {
        deps!();
        IntoNodeReferences ! { delegate_impl [[G] , G , UndirectedAdaptor < G >, access0] }
    };
}

macro_215!()