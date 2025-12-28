macro_rules! deps {
    () => {
        Edge!();
    };
}

macro_rules! impl_670 {
    () => {
        deps!();
        impl < E , Ix > Clone for Edge < E , Ix > where E : Clone , Ix : Copy , { clone_fields ! (Edge , weight , next , node ,) ; }
    };
}

impl_670!();