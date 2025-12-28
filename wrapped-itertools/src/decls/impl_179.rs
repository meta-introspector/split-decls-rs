macro_rules! deps {
    () => {
        CombinationsGeneric!();
    };
}

macro_rules! impl_179 {
    () => {
        deps!();
        impl < I , Idx > Clone for CombinationsGeneric < I , Idx > where I : Iterator + Clone , I :: Item : Clone , Idx : Clone , { clone_fields ! (indices , pool , first) ; }
    };
}

impl_179!();