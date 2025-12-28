macro_rules! deps {
    () => {
        StableGraphNode!();
    };
}

macro_rules! impl_804 {
    () => {
        deps!();
        impl < N , Ix > Clone for StableGraphNode < N , Ix > where N : Clone , Ix : Copy , { fn clone (& self) -> Self { Self { index : self . index , weight : self . weight . clone () , } } }
    };
}

impl_804!();