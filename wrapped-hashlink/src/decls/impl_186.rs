macro_rules! deps {
    () => {
        LruCache!();
    };
}

macro_rules! impl_186 {
    () => {
        deps!();
        impl < K : Hash + Eq + Clone , V : Clone , S : BuildHasher + Clone > Clone for LruCache < K , V , S > { # [inline] fn clone (& self) -> Self { LruCache { map : self . map . clone () , max_size : self . max_size , } } }
    };
}

impl_186!()