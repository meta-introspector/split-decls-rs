macro_rules! deps {
    () => {
        LinkedHashMap!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < K : Hash + Eq + Clone , V : Clone , S : BuildHasher + Clone > Clone for LinkedHashMap < K , V , S > { # [inline] fn clone (& self) -> Self { let mut map = Self :: with_hasher (self . hash_builder . clone ()) ; map . extend (self . iter () . map (| (k , v) | (k . clone () , v . clone ()))) ; map } }
    };
}

impl_20!();