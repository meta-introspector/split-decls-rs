macro_rules! deps {
    () => {
        EdgeType!();
        EdgeProperty!();
        Undirected!();
        Directed!();
    };
}

macro_rules! impl_633 {
    () => {
        deps!();
        impl < Ty > From < PhantomData < Ty > > for EdgeProperty where Ty : EdgeType , { fn from (_ : PhantomData < Ty >) -> Self { if Ty :: is_directed () { EdgeProperty :: Directed } else { EdgeProperty :: Undirected } } }
    };
}

impl_633!()