macro_rules! deps {
    () => {
        CoreMap!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl < K , V , const N : usize > Clone for CoreMap < K , V , N > where K : Clone , V : Clone , { fn clone (& self) -> Self { Self { entries : self . entries . clone () , indices : self . indices , } } }
    };
}

impl_91!()