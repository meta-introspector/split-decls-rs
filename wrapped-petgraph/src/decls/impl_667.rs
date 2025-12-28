macro_rules! deps {
    () => {
        Node!();
    };
}

macro_rules! impl_667 {
    () => {
        deps!();
        impl < E , Ix > Clone for Node < E , Ix > where E : Clone , Ix : Copy , { clone_fields ! (Node , weight , next ,) ; }
    };
}

impl_667!();