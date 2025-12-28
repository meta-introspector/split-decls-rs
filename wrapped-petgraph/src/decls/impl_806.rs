macro_rules! deps {
    () => {
        StableGraphEdge!();
    };
}

macro_rules! impl_806 {
    () => {
        deps!();
        impl < E , Ix > Clone for StableGraphEdge < E , Ix > where E : Clone , Ix : Copy , { fn clone (& self) -> Self { Self { index : self . index , source : self . source , target : self . target , weight : self . weight . clone () , } } }
    };
}

impl_806!()