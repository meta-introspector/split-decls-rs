macro_rules! deps {
    () => {
        Undirected!();
        Directed!();
        EdgeProperty!();
    };
}

macro_rules! impl_632 {
    () => {
        deps!();
        impl EdgeProperty { pub fn is_directed (& self) -> bool { match * self { EdgeProperty :: Directed => true , EdgeProperty :: Undirected => false , } } }
    };
}

impl_632!();