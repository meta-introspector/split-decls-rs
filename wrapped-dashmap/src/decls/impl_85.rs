macro_rules! deps {
    () => {
        ReadOnlyView!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl < K : Eq + Hash + Clone , V : Clone , S : Clone > Clone for ReadOnlyView < K , V , S > { fn clone (& self) -> Self { Self { map : self . map . clone () , } } }
    };
}

impl_85!();