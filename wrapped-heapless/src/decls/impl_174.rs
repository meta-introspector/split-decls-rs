macro_rules! deps {
    () => {
        LinearMap!();
    };
}

macro_rules! impl_174 {
    () => {
        deps!();
        impl < K , V , const N : usize > Clone for LinearMap < K , V , N > where K : Eq + Clone , V : Clone , { fn clone (& self) -> Self { Self { buffer : self . buffer . clone () , } } }
    };
}

impl_174!();