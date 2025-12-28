macro_rules! deps {
    () => {
        IndexMap!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl < K , V , S > Clone for IndexMap < K , V , S > where K : Clone , V : Clone , S : Clone , { fn clone (& self) -> Self { IndexMap { core : self . core . clone () , hash_builder : self . hash_builder . clone () , } } fn clone_from (& mut self , other : & Self) { self . core . clone_from (& other . core) ; self . hash_builder . clone_from (& other . hash_builder) ; } }
    };
}

impl_49!()