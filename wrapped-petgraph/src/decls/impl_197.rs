macro_rules! deps {
    () => {
        GraphRef!();
        UndirectedAdaptor!();
    };
}

macro_rules! impl_197 {
    () => {
        deps!();
        impl < G : GraphRef > GraphRef for UndirectedAdaptor < G > { }
    };
}

impl_197!()