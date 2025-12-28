macro_rules! deps {
    () => {
        Csr!();
        IndexType!();
        EdgeType!();
    };
}

macro_rules! impl_517 {
    () => {
        deps!();
        impl < N , E , Ty , Ix > Default for Csr < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { fn default () -> Self { Self :: new () } }
    };
}

impl_517!()