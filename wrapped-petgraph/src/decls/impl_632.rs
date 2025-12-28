macro_rules! deps {
    () => {
        EdgeProperty!();
        Undirected!();
        Directed!();
    };
}

macro_rules! impl_632 {
    () => {
        deps!();
        impl EdgeProperty { pub fn is_directed (& self) -> bool { match * self { EdgeProperty :: Directed => true , EdgeProperty :: Undirected => false , } } }
    };
}

impl_632!()