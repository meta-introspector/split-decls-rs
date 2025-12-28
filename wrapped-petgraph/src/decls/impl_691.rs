macro_rules! deps {
    () => {
        Neighbors!();
        IndexType!();
    };
}

macro_rules! impl_691 {
    () => {
        deps!();
        impl < E , Ix > Clone for Neighbors < '_ , E , Ix > where Ix : IndexType , { clone_fields ! (Neighbors , skip_start , edges , next ,) ; }
    };
}

impl_691!()