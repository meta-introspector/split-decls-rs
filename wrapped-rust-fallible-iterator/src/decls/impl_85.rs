macro_rules! deps {
    () => {
        FallibleIterator!();
        IntoFallibleIterator!();
        Flatten!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl < I > Clone for Flatten < I > where I : FallibleIterator + Clone , I :: Item : IntoFallibleIterator , < I :: Item as IntoFallibleIterator > :: IntoFallibleIter : Clone , { # [inline] fn clone (& self) -> Flatten < I > { Flatten { it : self . it . clone () , cur : self . cur . clone () , } } }
    };
}

impl_85!();