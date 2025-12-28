macro_rules! deps {
    () => {
        HeadTail!();
    };
}

macro_rules! impl_323 {
    () => {
        deps!();
        impl < I > Clone for HeadTail < I > where I : Iterator + Clone , I :: Item : Clone , { clone_fields ! (head , tail) ; }
    };
}

impl_323!();