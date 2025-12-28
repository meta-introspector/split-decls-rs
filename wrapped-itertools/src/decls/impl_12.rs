macro_rules! deps {
    () => {
        CoalesceBy!();
        CountItem!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl < I , F , C > Clone for CoalesceBy < I , F , C > where I : Clone + Iterator , F : Clone , C : CountItem < I :: Item > , C :: CItem : Clone , { clone_fields ! (last , iter , f) ; }
    };
}

impl_12!()